//! High-level game loop, persistence, and intent handling.

use crate::data::GameData;
use crate::state::{migrate_save_value, GameSession, Planet, SaveData};
use crate::ui::{self, UiAction, UiContext};
use macroquad::prelude::*;
use macroquad_toolkit::events::EventBus;
use macroquad_toolkit::notifications::{
    NotificationAnchor, NotificationManager, NotificationRenderConfig,
};
use macroquad_toolkit::persistence::{
    delete_slot, get_save_slots, load_from_slot_with_migration, save_to_slot_with_version,
    slot_exists,
};
use macroquad_toolkit::prelude::{begin_virtual_ui_frame, dark, end_virtual_ui_frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CaptureSceneSeed {
    reset_session: bool,
    game_started: bool,
    research_points: i64,
    first_research_complete: bool,
    research_open: bool,
    planet_demo: bool,
    reset_open: bool,
}

impl CaptureSceneSeed {
    fn for_scene(scene: &str) -> Self {
        match scene {
            "research" => Self {
                reset_session: true,
                game_started: true,
                research_points: 115,
                first_research_complete: true,
                research_open: true,
                planet_demo: false,
                reset_open: false,
            },
            "title" | "tutorial" => Self {
                reset_session: true,
                game_started: false,
                research_points: 0,
                first_research_complete: false,
                research_open: false,
                planet_demo: false,
                reset_open: false,
            },
            "biosphere" => Self {
                reset_session: true,
                game_started: true,
                research_points: 0,
                first_research_complete: false,
                research_open: false,
                planet_demo: true,
                reset_open: false,
            },
            "reset" => Self {
                reset_session: true,
                game_started: true,
                research_points: 0,
                first_research_complete: false,
                research_open: false,
                planet_demo: false,
                reset_open: true,
            },
            _ => Self {
                reset_session: false,
                game_started: false,
                research_points: 0,
                first_research_complete: false,
                research_open: false,
                planet_demo: false,
                reset_open: false,
            },
        }
    }
}

fn market_refresh_due(game_started: bool, elapsed: f32, refresh_seconds: f32) -> bool {
    game_started && refresh_seconds > 0.0 && elapsed >= refresh_seconds
}

pub struct Game {
    data: GameData,
    session: GameSession,
    notifications: NotificationManager,
    events: EventBus<UiAction>,
    save_exists: bool,
    save_slots: Vec<String>,
    tool_scroll: f32,
    market_scroll: f32,
    inventory_scroll: usize,
    expanded_tool: Option<String>,
    expanded_buyer: Option<u64>,
    history_open: bool,
    research_open: bool,
    reset_open: bool,
    market_elapsed: f32,
}

impl Game {
    pub async fn new() -> Self {
        let data = GameData::load().unwrap_or_else(|error| {
            panic!("Planet Trader embedded data failed to load: {}", error);
        });

        let mut game = Self {
            session: GameSession::new(&data),
            data,
            notifications: NotificationManager::new(),
            events: EventBus::new(),
            save_exists: false,
            save_slots: Vec::new(),
            tool_scroll: 0.0,
            market_scroll: 0.0,
            inventory_scroll: 0,
            expanded_tool: None,
            expanded_buyer: None,
            history_open: false,
            research_open: false,
            reset_open: false,
            market_elapsed: 0.0,
        };
        game.refresh_save_state();
        game.load_existing_save();
        game
    }

    pub fn begin_capture_scene(&mut self, scene: &str) {
        let seed = CaptureSceneSeed::for_scene(scene);
        self.research_open = false;
        self.reset_open = false;
        self.market_elapsed = 0.0;
        if !seed.reset_session {
            return;
        }

        self.session = GameSession::new(&self.data);
        self.session.game_started = seed.game_started;
        self.session.research_points = seed.research_points;
        if seed.first_research_complete {
            if let Some(research) = self.data.research.first() {
                self.session.completed_research.push(research.name.clone());
            }
        }
        if seed.planet_demo {
            if let Some(planet_type) = self.data.planet_types.first().cloned() {
                let planet = Planet {
                    id: "capture-planet".to_owned(),
                    planet_type,
                    name: "Verdant Relay".to_owned(),
                    temperature: 22.0,
                    atmosphere: 1.1,
                    water: 0.64,
                    gravity: 1.0,
                    radiation: 0.18,
                    biosphere: 1.0,
                    purchase_price: 1_800,
                    invested_cost: 2_200,
                    color: "#50A95A".to_owned(),
                };
                self.session.current_planet_id = Some(planet.id.clone());
                self.session.planets.push(planet);
            }
        }
        self.save_exists = false;
        self.save_slots.clear();
        self.reset_view();
        self.research_open = seed.research_open;
        self.reset_open = seed.reset_open;
    }

    pub fn update(&mut self, dt: f32) {
        self.notifications.update(dt);
        if self.session.game_started {
            self.market_elapsed += dt.max(0.0);
            if market_refresh_due(
                self.session.game_started,
                self.market_elapsed,
                self.data.config.buyer_refresh_seconds,
            ) {
                self.market_elapsed = 0.0;
                self.session.refresh_buyers(&self.data);
                self.expanded_buyer = None;
                self.market_scroll = 0.0;
                self.autosave();
                self.notifications.info("The alien market has refreshed");
            }
        } else {
            self.market_elapsed = 0.0;
        }

        if is_key_pressed(KeyCode::S) {
            self.events.push(UiAction::Save);
        }
        if is_key_pressed(KeyCode::L) {
            self.events.push(UiAction::Load);
        }

        let actions: Vec<UiAction> = self.events.drain().collect();
        for action in actions {
            self.apply_action(action);
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);
        let virtual_ui = begin_virtual_ui_frame(ui::LOGICAL_WIDTH, ui::LOGICAL_HEIGHT);
        let ctx = UiContext {
            data: &self.data,
            session: &self.session,
            save_exists: self.save_exists,
            save_slots: &self.save_slots,
            tool_scroll: self.tool_scroll,
            market_scroll: self.market_scroll,
            inventory_scroll: self.inventory_scroll,
            expanded_tool: self.expanded_tool.as_deref(),
            expanded_buyer: self.expanded_buyer,
            history_open: self.history_open,
            research_open: self.research_open,
            reset_open: self.reset_open,
            market_elapsed: self.market_elapsed,
            ui: &virtual_ui,
        };

        let actions = ui::draw_game_ui(ctx);
        end_virtual_ui_frame();
        for action in actions {
            self.events.push(action);
        }

        self.notifications.draw_with_config_and_offset(
            &NotificationRenderConfig {
                anchor: NotificationAnchor::TopRight,
                ..Default::default()
            },
            vec2(-490.0, 0.0),
        );
    }

    fn apply_action(&mut self, action: UiAction) {
        match action {
            UiAction::StartGame => {
                self.session.game_started = true;
                self.market_elapsed = 0.0;
                self.notifications
                    .info("Game started! Begin terraforming planets.");
                self.autosave();
            }
            UiAction::OpenPurchase => match self.session.open_purchase_modal(&self.data) {
                Ok(()) => {}
                Err(message) => self.notifications.danger(message),
            },
            UiAction::ClosePurchase => self.session.close_purchase_modal(),
            UiAction::PurchasePlanet(id) => match self.session.purchase_planet(&id) {
                Ok(message) => {
                    self.notifications.success(message);
                    self.autosave();
                }
                Err(message) => self.notifications.warning(message),
            },
            UiAction::SelectPlanet(id) => match self.session.select_planet(&id) {
                Ok(message) => {
                    self.notifications.info(message);
                    self.autosave();
                }
                Err(message) => self.notifications.warning(message),
            },
            UiAction::ToggleTool(id) => {
                self.expanded_tool = if self.expanded_tool.as_deref() == Some(id.as_str()) {
                    None
                } else {
                    Some(id)
                };
            }
            UiAction::ApplyTool(id) => {
                let tool = self
                    .data
                    .terraforming_tools
                    .iter()
                    .find(|tool| tool.id == id)
                    .cloned();
                if let Some(tool) = tool {
                    match self.session.apply_tool(&tool) {
                        Ok(message) => {
                            self.notifications.success(message);
                            self.autosave();
                        }
                        Err(message) => self.notifications.warning(message),
                    }
                }
            }
            UiAction::ToggleBuyer(id) => {
                self.expanded_buyer = if self.expanded_buyer == Some(id) {
                    None
                } else {
                    Some(id)
                };
            }
            UiAction::SellPlanet(id) => match self.session.sell_planet(id) {
                Ok(message) => {
                    self.notifications.success(message);
                    self.expanded_buyer = None;
                    self.autosave();
                }
                Err(message) => self.notifications.warning(message),
            },
            UiAction::ScrapPlanet => match self.session.salvage_current_planet() {
                Ok(message) => {
                    self.notifications.info(message);
                    self.expanded_buyer = None;
                    self.autosave();
                }
                Err(message) => self.notifications.warning(message),
            },
            UiAction::ToggleHistory => self.history_open = !self.history_open,
            UiAction::OpenResearch => self.research_open = true,
            UiAction::CloseResearch => self.research_open = false,
            UiAction::OpenResetConfirm => self.reset_open = true,
            UiAction::CancelReset => self.reset_open = false,
            UiAction::CompleteResearch(name) => {
                let research = self
                    .data
                    .research
                    .iter()
                    .find(|research| research.name == name)
                    .cloned();
                if let Some(research) = research {
                    match self.session.complete_research(&research) {
                        Ok(message) => {
                            self.notifications.success(message);
                            self.autosave();
                        }
                        Err(message) => self.notifications.warning(message),
                    }
                }
            }
            UiAction::ScrollTools(delta) => {
                self.tool_scroll = (self.tool_scroll + delta as f32 * 150.0).max(0.0);
            }
            UiAction::ScrollMarket(delta) => {
                self.market_scroll = (self.market_scroll + delta as f32 * 150.0).max(0.0);
            }
            UiAction::ScrollInventory(delta) => {
                let next = self.inventory_scroll as i32 + delta;
                self.inventory_scroll = next.max(0) as usize;
            }
            UiAction::Save => self.save_game(),
            UiAction::Load => self.load_game(),
            UiAction::DeleteSave => self.delete_save(),
        }
    }

    fn save_game(&mut self) {
        match self.write_save() {
            Ok(()) => {
                self.notifications.success("Progress saved");
                self.refresh_save_state();
            }
            Err(error) => self.notifications.danger(format!("Save failed: {}", error)),
        }
    }

    fn autosave(&mut self) {
        if let Err(error) = self.write_save() {
            self.notifications
                .warning(format!("Autosave unavailable: {}", error));
        }
        self.refresh_save_state();
    }

    fn write_save(&self) -> Result<(), String> {
        let save = self.session.to_save(&self.data.config.version);
        save_to_slot_with_version(
            &self.data.config.game_name,
            &self.data.config.save_slot,
            &save,
            &self.data.config.version,
        )
    }

    fn load_game(&mut self) {
        match self.read_save() {
            Ok(save) => {
                self.session = GameSession::from_save(save, &self.data);
                self.reset_view();
                self.notifications.success("Loaded saved progress");
                self.refresh_save_state();
            }
            Err(error) => self
                .notifications
                .warning(format!("Load failed: {}", error)),
        }
    }

    fn load_existing_save(&mut self) {
        if let Ok(save) = self.read_save() {
            self.session = GameSession::from_save(save, &self.data);
        }
    }

    fn read_save(&self) -> Result<SaveData, String> {
        let config = self.data.config.clone();
        load_from_slot_with_migration(
            &config.game_name,
            &config.save_slot,
            &config.version,
            |version, value| migrate_save_value(version, value, &config),
        )
    }

    fn delete_save(&mut self) {
        match delete_slot(&self.data.config.game_name, &self.data.config.save_slot) {
            Ok(()) => {
                self.session = GameSession::new(&self.data);
                self.reset_view();
                self.notifications.info("Saved progress deleted");
                self.refresh_save_state();
            }
            Err(error) => self
                .notifications
                .danger(format!("Delete failed: {}", error)),
        }
    }

    fn refresh_save_state(&mut self) {
        self.save_exists = slot_exists(&self.data.config.game_name, &self.data.config.save_slot);
        self.save_slots = get_save_slots(&self.data.config.game_name);
    }

    fn reset_view(&mut self) {
        self.tool_scroll = 0.0;
        self.market_scroll = 0.0;
        self.inventory_scroll = 0;
        self.expanded_tool = None;
        self.expanded_buyer = None;
        self.history_open = false;
        self.research_open = false;
        self.reset_open = false;
        self.market_elapsed = 0.0;
    }
}

#[cfg(test)]
mod tests;
