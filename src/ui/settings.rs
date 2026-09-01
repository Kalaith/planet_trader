use super::*;

pub(super) fn draw_settings(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.01, 0.025, 0.88),
    );
    let rect = Rect::new(170.0, 48.0, 940.0, 624.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.075, 0.11, 1.0))
            .with_border(2.0, Color::new(0.22, 0.70, 0.90, 1.0))
            .with_top_highlight(3.0, Color::new(0.38, 0.88, 1.0, 0.85)),
    );
    draw_ui_text_ex(
        "COMMAND SETTINGS",
        rect.x + 28.0,
        rect.y + 44.0,
        TextStyle::new(25.0, Color::new(0.48, 0.86, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Persistent accessibility, display, guidance, and company controls",
        rect.x + 30.0,
        rect.y + 67.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(rect.right() - 54.0, rect.y + 16.0, 34.0, 34.0),
        "X",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::CloseSettings);
    }

    draw_section_title("DISPLAY & READABILITY", rect.x + 30.0, rect.y + 112.0);
    setting_row(
        vec2(rect.x + 30.0, rect.y + 132.0),
        "Interface text",
        text_scale_label(ctx.settings.ui_text_scale),
        "Cycles between compact, standard, and large text.",
        UiAction::CycleTextScale,
        mouse,
        actions,
    );
    setting_row(
        vec2(rect.x + 30.0, rect.y + 204.0),
        "Fullscreen",
        on_off(ctx.settings.fullscreen),
        "Use the whole display; touch controls remain unchanged.",
        UiAction::ToggleFullscreen,
        mouse,
        actions,
    );
    setting_row(
        vec2(rect.x + 30.0, rect.y + 276.0),
        "Reduced motion",
        on_off(ctx.settings.reduced_motion),
        "Stops decorative home-screen movement.",
        UiAction::ToggleReducedMotion,
        mouse,
        actions,
    );
    setting_row(
        vec2(rect.x + 30.0, rect.y + 348.0),
        "Performance readout",
        on_off(ctx.settings.show_fps),
        "Shows the current frame rate in the lower edge.",
        UiAction::ToggleFps,
        mouse,
        actions,
    );

    draw_section_title("ORIENTATION & SAVES", rect.x + 492.0, rect.y + 112.0);
    if button(
        Rect::new(rect.x + 492.0, rect.y + 142.0, 382.0, 46.0),
        "RESTART GUIDED ORIENTATION",
        ctx.save_exists,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::RestartTutorial);
    }
    draw_text_block(
        "Replays the action-driven introduction from the first company step.",
        rect.x + 496.0,
        rect.y + 194.0,
        374.0,
        48.0,
        12.0,
        2.0,
        dark::TEXT_DIM,
    );
    if button(
        Rect::new(rect.x + 492.0, rect.y + 254.0, 382.0, 46.0),
        "RESET COMPANY SAVE",
        ctx.save_exists,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::OpenResetConfirm);
    }
    draw_text_block(
        "Opens a confirmation before erasing the current company charter.",
        rect.x + 496.0,
        rect.y + 306.0,
        374.0,
        48.0,
        12.0,
        2.0,
        dark::TEXT_DIM,
    );

    draw_section_title("CONTROLS", rect.x + 492.0, rect.y + 384.0);
    draw_text_block("Tap / click visible controls to navigate, buy, select, terraform, research, sell, and recover.\n\nMouse wheel may supplement the visible UP / DN controls. Save and Load are always available from the company command screen.", rect.x + 496.0, rect.y + 408.0, 370.0, 116.0, 13.0, 3.0, Color::new(0.67, 0.80, 0.84, 1.0));
    draw_ui_text_ex(
        "Planet Trader // Phase 1 Company Command",
        rect.x + 30.0,
        rect.bottom() - 24.0,
        TextStyle::new(11.0, Color::new(0.38, 0.64, 0.72, 1.0)).params(),
    );
}

fn draw_section_title(label: &str, x: f32, y: f32) {
    draw_ui_text_ex(
        label,
        x,
        y,
        TextStyle::new(13.0, Color::new(0.40, 0.82, 0.94, 1.0)).params(),
    );
}

fn setting_row(
    origin: Vec2,
    title: &str,
    value: &str,
    detail: &str,
    action: UiAction,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let rect = Rect::new(origin.x, origin.y, 420.0, 62.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
            .with_border(1.0, Color::new(0.20, 0.38, 0.46, 0.8)),
    );
    draw_ui_text_ex(
        title,
        rect.x + 12.0,
        rect.y + 23.0,
        TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        detail,
        rect.x + 12.0,
        rect.y + 45.0,
        TextStyle::new(10.5, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(rect.right() - 118.0, rect.y + 10.0, 106.0, 40.0),
        value,
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(action);
    }
}

fn text_scale_label(scale: f32) -> &'static str {
    if scale < 0.98 {
        "COMPACT"
    } else if scale > 1.05 {
        "LARGE"
    } else {
        "STANDARD"
    }
}

fn on_off(value: bool) -> &'static str {
    if value {
        "ON"
    } else {
        "OFF"
    }
}
