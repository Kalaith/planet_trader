use super::*;

pub(super) fn draw_home(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_starfield(ctx.settings.reduced_motion);
    draw_ui_text_ex(
        "WEB HATCHERY // FRONTIER COMMERCE",
        56.0,
        54.0,
        TextStyle::new(13.0, Color::new(0.42, 0.76, 0.88, 1.0)).params(),
    );
    draw_ui_text_ex(
        "PLANET",
        72.0,
        164.0,
        TextStyle::new(58.0, Color::new(0.86, 0.96, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "TRADER",
        72.0,
        222.0,
        TextStyle::new(58.0, Color::new(0.28, 0.80, 1.0, 1.0)).params(),
    );
    draw_text_block(
        "Acquire difficult worlds. Engineer them for alien life.\nBuild the frontier's most trusted planetary brokerage.",
        76.0,
        258.0,
        480.0,
        72.0,
        18.0,
        4.0,
        Color::new(0.64, 0.76, 0.82, 1.0),
    );

    let new_label = if ctx.save_exists {
        "NEW COMPANY"
    } else {
        "BEGIN NEW COMPANY"
    };
    if button(
        Rect::new(76.0, 364.0, 360.0, 58.0),
        new_label,
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::NewGame);
    }
    if button(
        Rect::new(76.0, 434.0, 360.0, 58.0),
        "CONTINUE COMPANY",
        ctx.save_exists,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::ContinueGame);
    }
    if button(
        Rect::new(76.0, 504.0, 174.0, 50.0),
        "SETTINGS",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::OpenSettings);
    }

    if ctx.save_exists {
        draw_company_summary(ctx);
    } else {
        draw_ui_text_ex(
            "No company charter found. Your first expedition awaits.",
            78.0,
            592.0,
            TextStyle::new(13.0, Color::new(0.42, 0.64, 0.72, 1.0)).params(),
        );
    }
    draw_hero_world(ctx.settings.reduced_motion);
    draw_ui_text_ex(
        "PHASE 1  //  COMPANY COMMAND",
        930.0,
        680.0,
        TextStyle::new(12.0, Color::new(0.36, 0.64, 0.72, 0.9)).params(),
    );
}

fn draw_starfield(reduced_motion: bool) {
    let time = if reduced_motion {
        0.0
    } else {
        get_time() as f32
    };
    for index in 0..72 {
        let seed = index as f32;
        let x = (seed * 83.37 + time * (2.0 + seed % 4.0)) % LOGICAL_WIDTH;
        let y = (seed * 47.11 + (seed * 1.7).sin() * 90.0) % LOGICAL_HEIGHT;
        let alpha = 0.18 + (seed * 0.73).sin().abs() * 0.48;
        draw_circle(
            x,
            y,
            0.7 + (index % 3) as f32 * 0.45,
            Color::new(0.5, 0.82, 1.0, alpha),
        );
    }
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.03, 0.06, 0.18),
    );
}

fn draw_hero_world(reduced_motion: bool) {
    let drift = if reduced_motion {
        0.0
    } else {
        (get_time() as f32 * 0.42).sin() * 8.0
    };
    let center = vec2(900.0, 350.0 + drift);
    for ring in (0..12).rev() {
        let radius = 132.0 + ring as f32 * 7.0;
        draw_circle(
            center.x,
            center.y,
            radius,
            Color::new(0.02, 0.30, 0.45, 0.018),
        );
    }
    draw_circle(center.x, center.y, 142.0, Color::new(0.02, 0.09, 0.14, 1.0));
    draw_circle(center.x, center.y, 136.0, Color::new(0.10, 0.46, 0.58, 1.0));
    draw_circle(
        center.x - 24.0,
        center.y - 18.0,
        112.0,
        Color::new(0.18, 0.68, 0.67, 0.88),
    );
    draw_circle(
        center.x + 36.0,
        center.y + 28.0,
        92.0,
        Color::new(0.06, 0.42, 0.62, 0.74),
    );
    draw_circle(
        center.x - 48.0,
        center.y - 54.0,
        42.0,
        Color::new(0.53, 0.79, 0.55, 0.72),
    );
    draw_circle_lines(
        center.x,
        center.y,
        145.0,
        3.0,
        Color::new(0.42, 0.90, 1.0, 0.92),
    );
    draw_circle_lines(
        center.x,
        center.y,
        186.0,
        1.0,
        Color::new(0.26, 0.65, 0.78, 0.34),
    );
    draw_circle(
        center.x + 178.0,
        center.y - 54.0,
        9.0,
        Color::new(0.82, 0.68, 0.42, 1.0),
    );
    draw_text_block(
        "THE FRONTIER IS NOT EMPTY.\nIT IS WAITING TO BE MADE HABITABLE.",
        735.0,
        570.0,
        410.0,
        52.0,
        15.0,
        3.0,
        Color::new(0.56, 0.78, 0.84, 1.0),
    );
}

fn draw_company_summary(ctx: &UiContext<'_>) {
    let rect = Rect::new(76.0, 578.0, 470.0, 74.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.12, 0.17, 0.94))
            .with_border(1.0, Color::new(0.20, 0.58, 0.72, 0.8)),
    );
    draw_ui_text_ex(
        "ACTIVE CHARTER",
        rect.x + 16.0,
        rect.y + 24.0,
        TextStyle::new(12.0, Color::new(0.40, 0.84, 0.94, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} CR   //   {} worlds   //   {} completed sales",
            ctx.session.credits,
            ctx.session.planets.len(),
            ctx.session.stats.planets_sold
        ),
        rect.x + 16.0,
        rect.y + 52.0,
        TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
    );
}

pub(super) fn draw_new_game_confirmation(mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.01, 0.03, 0.84),
    );
    let rect = Rect::new(330.0, 194.0, 620.0, 330.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.08, 0.12, 1.0))
            .with_border(2.0, Color::new(0.94, 0.58, 0.30, 1.0))
            .with_top_highlight(3.0, Color::new(1.0, 0.70, 0.36, 0.9)),
    );
    draw_ui_text_ex(
        "Found a New Company?",
        rect.x + 34.0,
        rect.y + 58.0,
        TextStyle::new(27.0, Color::new(1.0, 0.74, 0.40, 1.0)).params(),
    );
    draw_text_block(
        "A new charter replaces the current autosave. Your existing worlds, research, and trade history will be erased.",
        rect.x + 36.0,
        rect.y + 100.0,
        rect.w - 72.0,
        80.0,
        16.0,
        3.0,
        dark::TEXT,
    );
    if button(
        Rect::new(rect.x + 34.0, rect.bottom() - 78.0, 246.0, 48.0),
        "KEEP COMPANY",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::CancelNewGame);
    }
    if button(
        Rect::new(rect.right() - 280.0, rect.bottom() - 78.0, 246.0, 48.0),
        "START NEW",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::ConfirmNewGame);
    }
}
