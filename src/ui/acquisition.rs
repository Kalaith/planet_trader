use super::*;

pub(super) fn draw_acquisition(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let scan = Rect::new(18.0, 150.0, 804.0, 540.0);
    let pulse = Rect::new(838.0, 150.0, 424.0, 540.0);
    draw_surface(
        scan,
        &SurfaceStyle::new(Color::new(0.03, 0.075, 0.11, 1.0))
            .with_border(1.0, Color::new(0.20, 0.55, 0.70, 1.0))
            .with_left_accent(6.0, Color::new(0.28, 0.82, 1.0, 1.0)),
    );
    draw_ui_text_ex(
        "FRONTIER ACQUISITION",
        scan.x + 28.0,
        scan.y + 40.0,
        TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Find one difficult world with a credible route to four buyer requirements.",
        scan.x + 30.0,
        scan.y + 65.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    draw_scanner(vec2(scan.x + 390.0, scan.y + 260.0));
    draw_badge(
        Rect::new(scan.x + 230.0, scan.y + 390.0, 320.0, 38.0),
        &format!(
            "SCAN REACH  {} CONTRACTS",
            contract_option_count(ctx.session.reputation)
        ),
        Color::new(0.06, 0.15, 0.23, 1.0),
        Color::new(0.48, 0.82, 1.0, 1.0),
    );
    if button(
        Rect::new(scan.x + 214.0, scan.bottom() - 88.0, 352.0, 58.0),
        "SCAN FOR PLANET CONTRACTS",
        ctx.session.game_started,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenPurchase);
    }
    draw_ui_text_ex(
        "Scanning is free. Credits are committed only when you acquire a world.",
        scan.x + 218.0,
        scan.bottom() - 16.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );

    draw_panel(pulse, "MARKET PULSE");
    draw_ui_text_ex(
        "Buyer demand rotates. These are today's strongest base offers.",
        pulse.x + 16.0,
        pulse.y + 62.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    let mut buyers: Vec<_> = ctx.session.alien_buyers.iter().collect();
    buyers.sort_by_key(|buyer| std::cmp::Reverse(buyer.current_price));
    for (index, buyer) in buyers.into_iter().take(3).enumerate() {
        let card = Rect::new(
            pulse.x + 14.0,
            pulse.y + 82.0 + index as f32 * 92.0,
            pulse.w - 28.0,
            80.0,
        );
        draw_surface(
            card,
            &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
                .with_border(1.0, Color::new(0.17, 0.35, 0.42, 1.0)),
        );
        draw_circle(
            card.x + 18.0,
            card.y + 21.0,
            8.0,
            hex_to_color(&buyer.color),
        );
        draw_ui_text_ex(
            &buyer.name,
            card.x + 34.0,
            card.y + 24.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_right(
            &format!("{} CR", buyer.current_price),
            card.right() - 12.0,
            card.y + 24.0,
            TextStyle::new(13.0, Color::new(0.48, 1.0, 0.64, 1.0)),
        );
        draw_text_block(
            &buyer.description,
            card.x + 14.0,
            card.y + 40.0,
            card.w - 28.0,
            30.0,
            9.0,
            2.0,
            dark::TEXT_DIM,
        );
    }
    let reserve = Rect::new(
        pulse.x + 14.0,
        pulse.bottom() - 150.0,
        pulse.w - 28.0,
        126.0,
    );
    draw_surface(
        reserve,
        &SurfaceStyle::new(Color::new(0.05, 0.105, 0.14, 1.0))
            .with_border(1.0, Color::new(0.17, 0.35, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "BROKER RESERVE",
        reserve.x + 16.0,
        reserve.y + 28.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", ctx.session.credits),
        reserve.x + 16.0,
        reserve.y + 66.0,
        TextStyle::new(29.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_right(
        &format!("{} worlds owned", ctx.session.planets.len()),
        reserve.right() - 16.0,
        reserve.y + 62.0,
        TextStyle::new(11.0, dark::TEXT_DIM),
    );
    draw_ui_text_ex(
        "Keep enough capital for at least two coarse interventions.",
        reserve.x + 16.0,
        reserve.bottom() - 18.0,
        TextStyle::new(10.0, Color::new(1.0, 0.74, 0.38, 1.0)).params(),
    );
}

fn draw_scanner(center: Vec2) {
    for (index, radius) in [58.0, 104.0, 150.0].into_iter().enumerate() {
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            2.0,
            Color::new(0.20, 0.65, 0.82, 0.22 + index as f32 * 0.08),
        );
    }
    draw_line(
        center.x - 170.0,
        center.y,
        center.x + 170.0,
        center.y,
        1.0,
        Color::new(0.20, 0.55, 0.70, 0.3),
    );
    draw_line(
        center.x,
        center.y - 170.0,
        center.x,
        center.y + 170.0,
        1.0,
        Color::new(0.20, 0.55, 0.70, 0.3),
    );
    for (offset, size) in [
        (vec2(-92.0, -45.0), 9.0),
        (vec2(62.0, 88.0), 12.0),
        (vec2(118.0, -76.0), 7.0),
    ] {
        let point = center + offset;
        draw_circle(point.x, point.y, size, Color::new(0.36, 0.88, 1.0, 0.9));
        draw_circle_lines(
            point.x,
            point.y,
            size + 6.0,
            1.0,
            Color::new(0.36, 0.88, 1.0, 0.45),
        );
    }
    draw_circle(center.x, center.y, 24.0, Color::new(0.08, 0.26, 0.34, 1.0));
    draw_circle_lines(
        center.x,
        center.y,
        28.0,
        2.0,
        Color::new(0.40, 0.92, 1.0, 0.9),
    );
}
