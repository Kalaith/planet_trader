use super::*;

pub(super) fn draw_purchase_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.015, 0.03, 0.88),
    );
    let shell = Rect::new(42.0, 26.0, 1196.0, 668.0);
    draw_surface(
        shell,
        &SurfaceStyle::new(Color::new(0.035, 0.075, 0.105, 1.0))
            .with_border(2.0, Color::new(0.22, 0.70, 0.92, 1.0))
            .with_top_highlight(3.0, Color::new(0.40, 0.86, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "FRONTIER CONTRACT SCAN",
        shell.x + 22.0,
        shell.y + 36.0,
        TextStyle::new(23.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} offers  //  {} CR available  //  select a world to inspect",
            ctx.session.planet_options.len(),
            ctx.session.credits
        ),
        shell.x + 24.0,
        shell.y + 58.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(shell.right() - 52.0, shell.y + 14.0, 34.0, 34.0),
        "X",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ClosePurchase);
    }

    let rail = Rect::new(shell.x + 18.0, shell.y + 78.0, 294.0, shell.h - 98.0);
    let scan = Rect::new(rail.right() + 14.0, rail.y, 500.0, rail.h);
    let decision = Rect::new(
        scan.right() + 14.0,
        rail.y,
        shell.right() - scan.right() - 32.0,
        rail.h,
    );
    draw_surface(
        rail,
        &SurfaceStyle::new(Color::new(0.045, 0.095, 0.13, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_surface(
        scan,
        &SurfaceStyle::new(Color::new(0.025, 0.07, 0.105, 1.0))
            .with_border(1.0, Color::new(0.18, 0.44, 0.55, 1.0)),
    );
    draw_surface(
        decision,
        &SurfaceStyle::new(Color::new(0.045, 0.095, 0.13, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "CONTRACTS",
        rail.x + 14.0,
        rail.y + 26.0,
        TextStyle::new(13.0, Color::new(0.45, 0.82, 0.96, 1.0)).params(),
    );

    for (index, planet) in ctx.session.planet_options.iter().enumerate() {
        let card = Rect::new(
            rail.x + 10.0,
            rail.y + 40.0 + index as f32 * 98.0,
            rail.w - 20.0,
            88.0,
        );
        let selected = ctx.selected_offer == Some(planet.id.as_str());
        if button(
            card,
            &format!(
                "{}\n{}  //  {} CR",
                planet.name, planet.planet_type.name, planet.purchase_price
            ),
            true,
            if selected {
                ButtonTone::Primary
            } else {
                ButtonTone::Muted
            },
            mouse,
        ) {
            actions.push(UiAction::SelectOffer(planet.id.clone()));
        }
    }

    let selected = ctx
        .selected_offer
        .and_then(|id| {
            ctx.session
                .planet_options
                .iter()
                .find(|planet| planet.id == id)
        })
        .or_else(|| ctx.session.planet_options.first());
    let Some(planet) = selected else {
        draw_text_centered_in_box(
            "No viable contracts returned.",
            scan.x,
            scan.center().y,
            scan.w,
            30.0,
            17.0,
            dark::TEXT_DIM,
        );
        return;
    };
    draw_ui_text_ex(
        &planet.name,
        scan.x + 18.0,
        scan.y + 32.0,
        TextStyle::new(21.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} WORLD  //  BROKER SCAN",
            planet.planet_type.name.to_uppercase()
        ),
        scan.x + 20.0,
        scan.y + 52.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    draw_planet_orb(planet, vec2(scan.center().x, scan.y + 220.0), 132.0);
    draw_scan_stats(planet, scan);

    draw_ui_text_ex(
        "ACQUISITION ROUTE",
        decision.x + 16.0,
        decision.y + 28.0,
        TextStyle::new(14.0, Color::new(0.45, 0.82, 0.96, 1.0)).params(),
    );
    if let Some((buyer, score, estimate, profit)) =
        best_market_route(planet, &ctx.session.alien_buyers)
    {
        let matches = compatibility_matches(score);
        draw_ui_text_ex(
            &buyer.name,
            decision.x + 16.0,
            decision.y + 64.0,
            TextStyle::new(17.0, dark::TEXT_BRIGHT).params(),
        );
        draw_ui_text_ex(
            &format!("{} / 6 MATCH  //  {} CR", matches, estimate),
            decision.x + 16.0,
            decision.y + 88.0,
            TextStyle::new(
                13.0,
                if matches >= 4 {
                    Color::new(0.46, 0.94, 0.62, 1.0)
                } else {
                    Color::new(1.0, 0.72, 0.36, 1.0)
                },
            )
            .params(),
        );
        draw_ui_text_ex(
            &format!("Current route margin  {:+} CR", profit),
            decision.x + 16.0,
            decision.y + 112.0,
            TextStyle::new(12.0, dark::TEXT_DIM).params(),
        );
        draw_text_block("This is a low-confidence route estimate. Workshop interventions may improve the fit, but every tool spends reserve capital.", decision.x + 16.0, decision.y + 142.0, decision.w - 32.0, 72.0, 12.0, 4.0, dark::TEXT_DIM);
    }
    draw_ui_text_ex(
        "CAPITAL COMMITMENT",
        decision.x + 16.0,
        decision.y + 260.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", planet.purchase_price),
        decision.x + 16.0,
        decision.y + 302.0,
        TextStyle::new(32.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} CR remains for engineering",
            ctx.session.credits - planet.purchase_price
        ),
        decision.x + 16.0,
        decision.y + 328.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    let affordable = ctx.session.credits >= planet.purchase_price;
    if button(
        Rect::new(
            decision.x + 16.0,
            decision.bottom() - 72.0,
            decision.w - 32.0,
            52.0,
        ),
        if affordable {
            "ACQUIRE WORLD"
        } else {
            "INSUFFICIENT CAPITAL"
        },
        affordable,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::PurchasePlanet(planet.id.clone()));
    }
}

fn draw_scan_stats(planet: &Planet, scan: Rect) {
    let stats = [
        ("TEMPERATURE", format!("{:.0} C", planet.temperature)),
        ("ATMOSPHERE", format!("{:.2} ATM", planet.atmosphere)),
        ("SURFACE WATER", format!("{:.0}%", planet.water * 100.0)),
        ("GRAVITY", format!("{:.2} G", planet.gravity)),
        ("RADIATION", format!("{:.2} RAD", planet.radiation)),
        ("BIOSPHERE", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        let card = Rect::new(
            scan.x + 16.0 + column as f32 * 238.0,
            scan.y + 374.0 + row as f32 * 54.0,
            226.0,
            44.0,
        );
        draw_surface(
            card,
            &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
                .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
        );
        draw_ui_text_ex(
            label,
            card.x + 9.0,
            card.y + 17.0,
            TextStyle::new(9.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            value,
            card.right() - 9.0,
            card.y + 30.0,
            TextStyle::new(14.0, dark::TEXT_BRIGHT),
        );
    }
    draw_ui_text_ex(
        "Additional composition analysis locked",
        scan.x + 18.0,
        scan.bottom() - 18.0,
        TextStyle::new(10.0, Color::new(0.72, 0.60, 0.35, 1.0)).params(),
    );
}
