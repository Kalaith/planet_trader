use super::*;

pub(super) fn draw_deck(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_command_header(ctx, mouse, actions);
    draw_mode_bar(ctx, mouse, actions);

    match ctx.mode {
        GameplayMode::Acquire => acquisition::draw_acquisition(ctx, mouse, actions),
        GameplayMode::Workshop => workshop::draw_workshop(ctx, mouse, actions),
        GameplayMode::Market => market_deck::draw_market(ctx, mouse, actions),
        GameplayMode::Research => research::draw_research_page(ctx, mouse, actions),
        GameplayMode::Company => company::draw_company(ctx, mouse, actions),
    }
}

fn draw_command_header(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let style = SurfaceStyle::new(Color::new(0.025, 0.052, 0.080, 0.99))
        .with_border(1.0, Color::new(0.12, 0.31, 0.40, 0.9))
        .with_top_highlight(1.0, Color::new(0.28, 0.68, 0.82, 0.8));
    draw_surface(HEADER, &style);
    draw_ui_text_ex(
        "PLANET TRADER",
        HEADER.x + 18.0,
        HEADER.y + 27.0,
        TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "COMPANY COMMAND",
        HEADER.x + 20.0,
        HEADER.y + 44.0,
        TextStyle::new(8.0, Color::new(0.30, 0.72, 0.88, 1.0)).params(),
    );
    draw_ui_text_ex(
        "CAPITAL",
        276.0,
        27.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", ctx.session.credits),
        276.0,
        48.0,
        TextStyle::new(14.0, Color::new(0.48, 1.0, 0.64, 1.0)).params(),
    );
    draw_line(
        408.0,
        25.0,
        408.0,
        55.0,
        1.0,
        Color::new(0.20, 0.40, 0.46, 0.45),
    );
    draw_ui_text_ex(
        "REPUTATION",
        426.0,
        27.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} REP", ctx.session.reputation),
        426.0,
        48.0,
        TextStyle::new(14.0, Color::new(1.0, 0.78, 0.38, 1.0)).params(),
    );
    let active = ctx
        .session
        .current_planet()
        .map(|planet| planet.name.as_str())
        .unwrap_or("No active world");
    draw_line(
        538.0,
        25.0,
        538.0,
        55.0,
        1.0,
        Color::new(0.20, 0.40, 0.46, 0.45),
    );
    draw_ui_text_ex(
        "CURRENT WORLD",
        556.0,
        27.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        active,
        556.0,
        48.0,
        TextStyle::new(13.0, Color::new(0.48, 0.82, 1.0, 1.0)).params(),
    );
    let (rank, _) = company_rank(ctx.session.reputation);
    if button(
        Rect::new(914.0, 25.0, 82.0, 32.0),
        "SAVE",
        ctx.session.game_started,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(1004.0, 25.0, 102.0, 32.0),
        "SETTINGS",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::OpenSettings);
    }
    if button(
        Rect::new(1114.0, 25.0, 124.0, 32.0),
        "HOME",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ReturnHome);
    }
    draw_text_right(
        rank,
        890.0,
        46.0,
        TextStyle::new(9.0, Color::new(0.68, 0.60, 0.42, 1.0)),
    );
}

fn draw_mode_bar(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_surface(
        MODE_BAR,
        &SurfaceStyle::new(Color::new(0.035, 0.065, 0.095, 0.98))
            .with_border(1.0, Color::new(0.11, 0.25, 0.32, 1.0)),
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
            MODE_BAR.y + 4.0,
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
