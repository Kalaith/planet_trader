//! Shared UI types and helpers for the Planet Trader renderer.

use crate::data::{GameData, Tool};
use crate::state::{
    analysis_level, company_rank, compatibility, compatibility_matches, contract_option_count,
    forecast_range, knowledge_award_for_matches, market_trend_percent, potential_profit,
    projected_research_points, sale_price, tool_cost, tool_is_locked, AlienBuyer, GameSession,
    Planet, ToolIntensity, TutorialStep, KNOWLEDGE_FIELDS,
};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::settings::GameSettings;
use macroquad_toolkit::ui::draw_ui_text_ex;
use macroquad_toolkit::ui::{button_rect_tone_at, RectExt, VirtualUi};
use planet_graphics::{draw_planet_gallery, draw_planet_orb};

mod acquisition;
mod company;
mod deck;
mod home;
mod market_deck;
mod overlays;
mod planet_graphics;
mod research;
mod settings;
mod tutorial;
mod workshop;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

const HEADER: Rect = Rect::new(18.0, 16.0, 1244.0, 64.0);
const MODE_BAR: Rect = Rect::new(18.0, 88.0, 1244.0, 48.0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    Gameplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplayMode {
    Acquire,
    Workshop,
    Market,
    Research,
    Company,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiAction {
    NewGame,
    ConfirmNewGame,
    CancelNewGame,
    ContinueGame,
    ReturnHome,
    BeginTutorial,
    OpenSettings,
    CloseSettings,
    CycleTextScale,
    ToggleFullscreen,
    ToggleReducedMotion,
    ToggleFps,
    RestartTutorial,
    SetMode(GameplayMode),
    OpenPurchase,
    ClosePurchase,
    PurchasePlanet(String),
    SelectOffer(String),
    SelectPlanet(String),
    ToggleTool(String),
    ApplyTool(String),
    SetToolIntensity(ToolIntensity),
    ToggleBuyer(u64),
    SellPlanet(u64),
    ScrapPlanet,
    ToggleHistory,
    CloseResearch,
    CompleteResearch(String),
    SetResearchBranch(String),
    OpenResetConfirm,
    CancelReset,
    ScrollTools(i32),
    ScrollInventory(i32),
    Save,
    Load,
    DeleteSave,
}

pub struct UiContext<'a> {
    pub data: &'a GameData,
    pub session: &'a GameSession,
    pub save_exists: bool,
    pub tool_scroll: f32,
    pub inventory_scroll: usize,
    pub selected_offer: Option<&'a str>,
    pub expanded_tool: Option<&'a str>,
    pub tool_intensity: ToolIntensity,
    pub expanded_buyer: Option<u64>,
    pub history_open: bool,
    pub research_open: bool,
    pub research_branch: &'a str,
    pub reset_open: bool,
    pub screen: AppScreen,
    pub settings_open: bool,
    pub new_game_confirm: bool,
    pub settings: &'a GameSettings,
    pub mode: GameplayMode,
    pub planet_gallery: bool,
    pub ui: &'a VirtualUi,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ctx.ui.mouse_position();

    if ctx.planet_gallery {
        draw_planet_gallery(&ctx);
        return actions;
    }

    match ctx.screen {
        AppScreen::Home => home::draw_home(&ctx, mouse, &mut actions),
        AppScreen::Gameplay => {
            deck::draw_deck(&ctx, mouse, &mut actions);
            if ctx.session.planet_modal_open {
                actions.clear();
                overlays::draw_purchase_modal(&ctx, mouse, &mut actions);
            }
            if ctx.history_open {
                actions.clear();
                overlays::draw_history_modal(&ctx, mouse, &mut actions);
            }
            if ctx.research_open {
                actions.clear();
                research::draw_research_modal(&ctx, mouse, &mut actions);
            }
            if !ctx.session.tutorial_step.is_complete() {
                if ctx.session.tutorial_step == TutorialStep::Welcome {
                    actions.clear();
                }
                tutorial::draw_tutorial(&ctx, mouse, &mut actions);
            }
        }
    }
    if ctx.settings_open {
        actions.clear();
        settings::draw_settings(&ctx, mouse, &mut actions);
    }
    if ctx.new_game_confirm {
        actions.clear();
        home::draw_new_game_confirmation(mouse, &mut actions);
    }
    if ctx.reset_open {
        actions.clear();
        overlays::draw_reset_confirmation(mouse, &mut actions);
    }
    if ctx.screen == AppScreen::Gameplay
        && ctx.session.game_started
        && !ctx.session.planet_modal_open
        && !ctx.history_open
        && !ctx.research_open
        && !ctx.reset_open
        && !ctx.settings_open
        && !ctx.new_game_confirm
        && ctx.session.tutorial_step != TutorialStep::Welcome
    {
        add_wheel_scroll_action(mouse, &mut actions);
    }
    actions
}

fn add_wheel_scroll_action(mouse: Vec2, actions: &mut Vec<UiAction>) {
    let (_, wheel_y) = mouse_wheel();
    let Some(delta) = wheel_scroll_delta(wheel_y) else {
        return;
    };

    if mouse.x < 380.0 && mouse.y >= 150.0 {
        actions.push(UiAction::ScrollTools(delta));
    } else if mouse.x >= 394.0 && mouse.x <= 934.0 && mouse.y >= 580.0 {
        actions.push(UiAction::ScrollInventory(delta));
    }
}

fn wheel_scroll_delta(wheel_y: f32) -> Option<i32> {
    if wheel_y > 0.0 {
        Some(-1)
    } else if wheel_y < 0.0 {
        Some(1)
    } else {
        None
    }
}
fn draw_panel(rect: Rect, title: &str) {
    let style = SurfaceStyle::new(Color::new(0.045, 0.09, 0.13, 0.98))
        .with_border(1.0, Color::new(0.20, 0.37, 0.44, 1.0))
        .with_header(42.0, Color::new(0.065, 0.13, 0.18, 1.0))
        .with_header_divider(1.0, Color::new(0.21, 0.40, 0.48, 0.85));
    draw_surface_with_title(
        rect,
        Some(title),
        &style,
        TextStyle::new(18.0, Color::new(0.50, 0.82, 0.96, 1.0)),
    );
}

fn draw_panel_with_left_title(rect: Rect, title: &str) {
    let style = SurfaceStyle::new(Color::new(0.045, 0.09, 0.13, 0.98))
        .with_border(1.0, Color::new(0.20, 0.37, 0.44, 1.0))
        .with_header(42.0, Color::new(0.065, 0.13, 0.18, 1.0))
        .with_header_divider(1.0, Color::new(0.21, 0.40, 0.48, 0.85));
    draw_surface(rect, &style);
    draw_ui_text_ex(
        title,
        rect.x + 14.0,
        rect.y + 28.0,
        TextStyle::new(18.0, Color::new(0.50, 0.82, 0.96, 1.0)).params(),
    );
}

fn button(rect: Rect, text: &str, enabled: bool, tone: ButtonTone, mouse: Vec2) -> bool {
    button_rect_tone_at(rect, text, enabled, tone, mouse)
}

fn tool_icon(category: &str) -> &'static str {
    match category {
        "temperature" => "T",
        "atmosphere" => "A",
        "water" => "W",
        "gravity" => "G",
        "radiation" => "R",
        "biological" => "B",
        _ => "?",
    }
}

fn effect_forecast(
    effects: &std::collections::HashMap<String, f32>,
    intensity: ToolIntensity,
    analysis: u32,
) -> String {
    let mut entries: Vec<_> = effects.iter().collect();
    entries.sort_by(|left, right| left.0.cmp(right.0));
    if entries.is_empty() {
        return "none".to_owned();
    }
    entries
        .into_iter()
        .map(|(stat, value)| {
            let adjusted = value * intensity.multiplier();
            let (low, high) = forecast_range(adjusted, analysis);
            if analysis >= 3 {
                format!("{} {:+.2}", stat, adjusted)
            } else {
                format!("{} {:+.2} to {:+.2}", stat, low.min(high), low.max(high))
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn best_market_route<'a>(
    planet: &Planet,
    buyers: &'a [AlienBuyer],
) -> Option<(&'a AlienBuyer, f32, i64, i64)> {
    buyers
        .iter()
        .map(|buyer| {
            let score = compatibility(planet, buyer);
            let price = sale_price(planet, buyer);
            (buyer, score, price, price - planet.invested_cost)
        })
        .max_by(|left, right| {
            left.1
                .total_cmp(&right.1)
                .then_with(|| left.3.cmp(&right.3))
        })
}

fn hex_to_color(value: &str) -> Color {
    let cleaned = value.trim().trim_start_matches('#');
    let expanded = if cleaned.len() == 3 {
        cleaned
            .chars()
            .flat_map(|character| [character, character])
            .collect::<String>()
    } else {
        cleaned.to_owned()
    };
    if expanded.len() != 6
        || !expanded
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Color::new(0.35, 0.45, 0.55, 1.0);
    }
    let red = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(90) as f32 / 255.0;
    let green = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(115) as f32 / 255.0;
    let blue = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(140) as f32 / 255.0;
    Color::new(red, green, blue, 1.0)
}

#[cfg(test)]
mod tests;
