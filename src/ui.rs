//! Shared UI types and helpers for the Planet Trader renderer.

use crate::data::{GameData, Tool};
use crate::state::{
    company_rank, compatibility, contract_option_count, market_trend_percent, potential_profit,
    projected_research_points, sale_price, tool_is_locked, AlienBuyer, GameSession, Planet,
    TutorialStep,
};
use macroquad::prelude::*;
use macroquad_toolkit::prelude::*;
use macroquad_toolkit::settings::GameSettings;
use macroquad_toolkit::ui::draw_ui_text_ex;
use macroquad_toolkit::ui::{button_rect_tone_at, RectExt, VirtualUi};

mod acquisition;
mod company;
mod deck;
mod home;
mod market;
mod overlays;
mod panels;
mod research;
mod settings;
mod tutorial;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;

const HEADER: Rect = Rect::new(18.0, 16.0, 1244.0, 64.0);
const MODE_BAR: Rect = Rect::new(18.0, 88.0, 1244.0, 48.0);
const TOOLS_PANEL: Rect = Rect::new(18.0, 150.0, 300.0, 540.0);
const CENTER_PANEL: Rect = Rect::new(332.0, 150.0, 596.0, 540.0);
const MARKET_PANEL: Rect = Rect::new(942.0, 150.0, 320.0, 540.0);
const INVENTORY_PANEL: Rect = Rect::new(332.0, 440.0, 596.0, 250.0);

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
    SelectPlanet(String),
    ToggleTool(String),
    ApplyTool(String),
    ToggleBuyer(u64),
    SellPlanet(u64),
    ScrapPlanet,
    ToggleHistory,
    OpenResearch,
    CloseResearch,
    CompleteResearch(String),
    OpenResetConfirm,
    CancelReset,
    ScrollTools(i32),
    ScrollMarket(i32),
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
    pub market_scroll: f32,
    pub inventory_scroll: usize,
    pub expanded_tool: Option<&'a str>,
    pub expanded_buyer: Option<u64>,
    pub history_open: bool,
    pub research_open: bool,
    pub reset_open: bool,
    pub screen: AppScreen,
    pub settings_open: bool,
    pub new_game_confirm: bool,
    pub settings: &'a GameSettings,
    pub mode: GameplayMode,
    pub market_elapsed: f32,
    pub ui: &'a VirtualUi,
}

pub fn draw_game_ui(ctx: UiContext<'_>) -> Vec<UiAction> {
    let mut actions = Vec::new();
    let mouse = ctx.ui.mouse_position();

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

    if TOOLS_PANEL.contains_point(mouse) {
        actions.push(UiAction::ScrollTools(delta));
    } else if INVENTORY_PANEL.contains_point(mouse) {
        actions.push(UiAction::ScrollInventory(delta));
    } else if MARKET_PANEL.contains_point(mouse) {
        actions.push(UiAction::ScrollMarket(delta));
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

fn tool_card_height(expanded: bool) -> f32 {
    if expanded {
        190.0
    } else {
        70.0
    }
}

fn max_tool_scroll(data: &GameData, expanded_tool: Option<&str>) -> f32 {
    let total: f32 = data
        .terraforming_tools
        .iter()
        .map(|tool| tool_card_height(expanded_tool == Some(tool.id.as_str())) + 8.0)
        .sum();
    (total - (TOOLS_PANEL.h - 82.0)).max(0.0)
}

fn max_market_scroll(ctx: &UiContext<'_>) -> f32 {
    let total: f32 = ctx
        .session
        .alien_buyers
        .iter()
        .map(|buyer| {
            (if ctx.expanded_buyer == Some(buyer.id) {
                210.0
            } else {
                92.0
            }) + 8.0
        })
        .sum();
    ((total - 8.0).max(0.0) - (MARKET_PANEL.h - 144.0)).max(0.0)
}

fn draw_scroll_hint(rect: Rect, can_scroll: bool) {
    if can_scroll {
        draw_text_centered_in_box(
            "Tap UP / DN to browse",
            rect.x,
            rect.bottom() - 20.0,
            rect.w,
            18.0,
            10.0,
            Color::new(0.40, 0.62, 0.70, 0.9),
        );
    }
}

fn draw_planet_orb(planet: &Planet, center: Vec2, radius: f32) {
    let color = hex_to_color(&planet.color);
    draw_circle(
        center.x,
        center.y,
        radius,
        Color::new(0.02, 0.035, 0.05, 1.0),
    );
    draw_circle(center.x, center.y, radius - 3.0, color);
    let atmosphere_alpha = (planet.atmosphere / 2.0).clamp(0.0, 0.9);
    draw_circle(
        center.x - radius * 0.12,
        center.y - radius * 0.12,
        radius * 0.88,
        Color::new(0.70, 0.82, 1.0, atmosphere_alpha),
    );
    let water_alpha = planet.water.clamp(0.0, 0.85);
    draw_circle(
        center.x + radius * 0.08,
        center.y + radius * 0.10,
        radius * 0.78,
        Color::new(0.08, 0.78, 0.88, water_alpha),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        3.0,
        Color::new(0.52, 0.82, 0.98, 1.0),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius - 8.0,
        1.0,
        Color::new(1.0, 1.0, 1.0, 0.22),
    );
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

fn effect_summary(effects: &std::collections::HashMap<String, f32>) -> String {
    let mut entries: Vec<_> = effects.iter().collect();
    entries.sort_by(|left, right| left.0.cmp(right.0));
    if entries.is_empty() {
        "none".to_owned()
    } else {
        entries
            .into_iter()
            .map(|(stat, value)| format!("{} {:+.2}", stat, value))
            .collect::<Vec<_>>()
            .join(", ")
    }
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
