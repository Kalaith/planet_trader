use super::*;

pub(super) fn draw_deck(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_command_header(ctx, mouse, actions);
    draw_mode_bar(ctx, mouse, actions);

    match ctx.mode {
        GameplayMode::Acquire => acquisition::draw_acquisition(ctx, mouse, actions),
        GameplayMode::Workshop => panels::draw_workshop_mode(ctx, mouse, actions),
        GameplayMode::Market => market::draw_market_mode(ctx, mouse, actions),
        GameplayMode::Research => research::draw_research_page(ctx, mouse, actions),
        GameplayMode::Company => company::draw_company(ctx, mouse, actions),
    }
}

fn draw_command_header(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let style = SurfaceStyle::new(Color::new(0.042, 0.068, 0.105, 0.99))
        .with_border(1.0, Color::new(0.16, 0.48, 0.66, 0.9))
        .with_top_highlight(2.0, Color::new(0.34, 0.80, 1.0, 0.9));
    draw_surface(HEADER, &style);
    draw_ui_text_ex(
        "PLANET TRADER",
        HEADER.x + 18.0,
        HEADER.y + 29.0,
        TextStyle::new(23.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "COMPANY COMMAND",
        HEADER.x + 20.0,
        HEADER.y + 50.0,
        TextStyle::new(11.0, Color::new(0.30, 0.72, 0.88, 1.0)).params(),
    );

    draw_badge(
        Rect::new(270.0, 30.0, 162.0, 34.0),
        &format!("{} CR", ctx.session.credits),
        Color::new(0.06, 0.24, 0.16, 1.0),
        Color::new(0.48, 1.0, 0.64, 1.0),
    );
    draw_badge(
        Rect::new(442.0, 30.0, 116.0, 34.0),
        &format!("{} RP", ctx.session.research_points),
        Color::new(0.12, 0.16, 0.30, 1.0),
        Color::new(0.67, 0.77, 1.0, 1.0),
    );
    let active = ctx
        .session
        .current_planet()
        .map(|planet| planet.name.as_str())
        .unwrap_or("No active world");
    draw_badge(
        Rect::new(568.0, 30.0, 250.0, 34.0),
        active,
        Color::new(0.06, 0.15, 0.23, 1.0),
        Color::new(0.48, 0.82, 1.0, 1.0),
    );
    let (rank, _) = company_rank(ctx.session.reputation);
    draw_badge(
        Rect::new(828.0, 30.0, 90.0, 34.0),
        &format!("{} REP", ctx.session.reputation),
        Color::new(0.22, 0.15, 0.07, 1.0),
        Color::new(1.0, 0.78, 0.38, 1.0),
    );
    if button(
        Rect::new(928.0, 28.0, 84.0, 38.0),
        "SAVE",
        ctx.session.game_started,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(1020.0, 28.0, 98.0, 38.0),
        "SETTINGS",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::OpenSettings);
    }
    if button(
        Rect::new(1126.0, 28.0, 112.0, 38.0),
        "HOME",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ReturnHome);
    }
    draw_ui_text_ex(
        rank,
        HEADER.x + 828.0,
        HEADER.bottom() - 5.0,
        TextStyle::new(8.0, Color::new(0.68, 0.60, 0.42, 1.0)).params(),
    );
}

fn draw_mode_bar(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_surface(
        MODE_BAR,
        &SurfaceStyle::new(Color::new(0.035, 0.065, 0.095, 0.98))
            .with_border(1.0, Color::new(0.13, 0.30, 0.39, 1.0)),
    );
    let modes = [
        (GameplayMode::Acquire, "ACQUIRE"),
        (GameplayMode::Workshop, "WORKSHOP"),
        (GameplayMode::Market, "ALIEN MARKET"),
        (GameplayMode::Research, "RESEARCH"),
        (GameplayMode::Company, "COMPANY"),
    ];
    for (index, (mode, label)) in modes.into_iter().enumerate() {
        let tab = Rect::new(
            MODE_BAR.x + 8.0 + index as f32 * 170.0,
            MODE_BAR.y + 7.0,
            160.0,
            34.0,
        );
        if button(
            tab,
            label,
            true,
            if ctx.mode == mode {
                ButtonTone::Primary
            } else {
                ButtonTone::Muted
            },
            mouse,
        ) {
            actions.push(UiAction::SetMode(mode));
        }
    }
    draw_text_right(
        if ctx.save_exists {
            "AUTOSAVE READY"
        } else {
            "NEW CHARTER"
        },
        MODE_BAR.right() - 18.0,
        MODE_BAR.y + 30.0,
        TextStyle::new(11.0, Color::new(0.42, 0.70, 0.78, 1.0)),
    );
}
