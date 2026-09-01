use super::*;

pub(super) fn draw_purchase_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.015, 0.03, 0.84),
    );
    let rect = Rect::new(80.0, 34.0, 1120.0, 650.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.09, 0.13, 1.0))
            .with_border(2.0, Color::new(0.22, 0.70, 0.92, 1.0))
            .with_top_highlight(3.0, Color::new(0.40, 0.86, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "Frontier Contract Scan",
        rect.x + 24.0,
        rect.y + 40.0,
        TextStyle::new(24.0, Color::new(0.48, 0.84, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!("Your credits: {} CR", ctx.session.credits),
        rect.x + 26.0,
        rect.y + 62.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(rect.right() - 54.0, rect.y + 16.0, 34.0, 34.0),
        "X",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ClosePurchase);
    }

    let offer_count = ctx.session.planet_options.len().max(1);
    let card_gap = 8.0;
    let card_height = ((530.0 - card_gap * (offer_count.saturating_sub(1)) as f32)
        / offer_count as f32)
        .clamp(94.0, 148.0);
    for (index, planet) in ctx.session.planet_options.iter().enumerate() {
        let card = Rect::new(
            rect.x + 24.0,
            rect.y + 82.0 + index as f32 * (card_height + card_gap),
            rect.w - 48.0,
            card_height,
        );
        draw_surface(
            card,
            &SurfaceStyle::new(Color::new(0.075, 0.14, 0.19, 1.0))
                .with_border(1.0, Color::new(0.24, 0.42, 0.50, 1.0)),
        );
        draw_circle(
            card.x + 48.0,
            card.center().y,
            27.0,
            hex_to_color(&planet.color),
        );
        draw_circle_lines(card.x + 48.0, card.center().y, 27.0, 2.0, dark::TEXT);
        draw_ui_text_ex(
            &planet.name,
            card.x + 94.0,
            card.y + 24.0,
            TextStyle::new(18.0, dark::TEXT).params(),
        );
        draw_ui_text_ex(
            &format!("Type: {}", planet.planet_type.name),
            card.x + 94.0,
            card.y + 43.0,
            TextStyle::new(12.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            &format!(
                "T {:.0}°C   A {:.1}x   W {:.0}%   G {:.1}x   R {:.1}x   B {:.1}",
                planet.temperature,
                planet.atmosphere,
                planet.water * 100.0,
                planet.gravity,
                planet.radiation,
                planet.biosphere
            ),
            card.x + 94.0,
            card.y + 63.0,
            TextStyle::new(12.0, Color::new(0.60, 0.78, 0.82, 1.0)).params(),
        );
        if let Some((buyer, score, estimate, profit)) =
            best_market_route(planet, &ctx.session.alien_buyers)
        {
            draw_ui_text_ex(
                &format!(
                    "Best route: {}  |  {:.0}%  |  {} CR  |  {:+} CR",
                    buyer.name,
                    score * 100.0,
                    estimate,
                    profit
                ),
                card.x + 94.0,
                card.y + 84.0,
                TextStyle::new(
                    11.0,
                    if score >= 0.6 {
                        Color::new(0.46, 0.94, 0.62, 1.0)
                    } else {
                        Color::new(1.0, 0.72, 0.36, 1.0)
                    },
                )
                .params(),
            );
        }
        let affordable = ctx.session.credits >= planet.purchase_price;
        draw_text_right(
            &format!("{} CR", planet.purchase_price),
            card.right() - 116.0,
            card.y + 28.0,
            TextStyle::new(18.0, Color::new(0.48, 1.0, 0.62, 1.0)),
        );
        if button(
            Rect::new(card.right() - 106.0, card.y + 46.0, 90.0, 36.0),
            if affordable { "BUY" } else { "TOO EXPENSIVE" },
            affordable,
            ButtonTone::Positive,
            mouse,
        ) {
            actions.push(UiAction::PurchasePlanet(planet.id.clone()));
        }
    }
    draw_ui_text_ex(
        "Routes compare the unmodified world against today's buyers. Orange routes need engineering before they can sell.",
        rect.x + 26.0,
        rect.bottom() - 20.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
}

pub(super) fn draw_reset_confirmation(mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.01, 0.02, 0.88),
    );
    let rect = Rect::new(330.0, 190.0, 620.0, 340.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.08, 0.07, 0.11, 1.0))
            .with_border(2.0, Color::new(0.85, 0.38, 0.38, 1.0))
            .with_top_highlight(3.0, Color::new(1.0, 0.50, 0.44, 0.8)),
    );
    draw_ui_text_ex(
        "Reset Progress?",
        rect.x + 30.0,
        rect.y + 58.0,
        TextStyle::new(27.0, Color::new(1.0, 0.64, 0.58, 1.0)).params(),
    );
    draw_text_block(
        "This will erase your credits, planets, research, and trade history.\nThis action cannot be undone.",
        rect.x + 34.0,
        rect.y + 96.0,
        rect.w - 68.0,
        70.0,
        16.0,
        3.0,
        dark::TEXT,
    );
    if button(
        Rect::new(rect.x + 34.0, rect.bottom() - 78.0, 246.0, 48.0),
        "KEEP PLAYING",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::CancelReset);
    }
    if button(
        Rect::new(rect.right() - 280.0, rect.bottom() - 78.0, 246.0, 48.0),
        "RESET PROGRESS",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::DeleteSave);
    }
}

pub(super) fn draw_history_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.015, 0.03, 0.84),
    );
    let rect = Rect::new(250.0, 92.0, 780.0, 536.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.09, 0.13, 1.0))
            .with_border(2.0, Color::new(0.22, 0.70, 0.92, 1.0))
            .with_top_highlight(3.0, Color::new(0.40, 0.86, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "Trade History",
        rect.x + 24.0,
        rect.y + 40.0,
        TextStyle::new(24.0, Color::new(0.48, 0.84, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} purchases | {} sales | {} CR realized profit | {} research nodes",
            ctx.session.stats.planets_purchased,
            ctx.session.stats.planets_sold,
            ctx.session.stats.total_profit,
            ctx.session.completed_research.len()
        ),
        rect.x + 26.0,
        rect.y + 64.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(rect.right() - 54.0, rect.y + 16.0, 34.0, 34.0),
        "X",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::ToggleHistory);
    }

    let rows = Rect::new(rect.x + 24.0, rect.y + 88.0, rect.w - 48.0, 392.0);
    if ctx.session.trade_history.is_empty() {
        draw_text_centered_in_box(
            "No trades recorded yet.",
            rows.x,
            rows.y + 150.0,
            rows.w,
            30.0,
            17.0,
            dark::TEXT_DIM,
        );
    } else {
        for (index, trade) in ctx.session.trade_history.iter().rev().take(8).enumerate() {
            let row = Rect::new(rows.x, rows.y + index as f32 * 48.0, rows.w, 40.0);
            let positive = trade.profit >= 0;
            draw_surface(
                row,
                &SurfaceStyle::new(Color::new(0.07, 0.13, 0.18, 1.0))
                    .with_border(1.0, Color::new(0.22, 0.35, 0.42, 1.0)),
            );
            draw_ui_text_ex(
                &format!(
                    "{}  {}",
                    trade.transaction_type.to_uppercase(),
                    trade.planet_name
                ),
                row.x + 12.0,
                row.y + 17.0,
                TextStyle::new(12.0, dark::TEXT).params(),
            );
            let counterparty = trade.buyer_name.as_deref().unwrap_or("Planet market");
            draw_ui_text_ex(
                counterparty,
                row.x + 12.0,
                row.y + 32.0,
                TextStyle::new(10.0, dark::TEXT_DIM).params(),
            );
            draw_text_right(
                &format!(
                    "Cash {}{} CR | P/L {}{} CR",
                    if trade.transaction_type == "purchase" {
                        "-"
                    } else {
                        "+"
                    },
                    if trade.transaction_type == "purchase" {
                        trade.purchase_cost
                    } else {
                        trade.sale_price
                    },
                    if trade.profit >= 0 { "+" } else { "" },
                    trade.profit
                ),
                row.right() - 12.0,
                row.y + 25.0,
                TextStyle::new(
                    12.0,
                    if positive {
                        Color::new(0.48, 0.94, 0.62, 1.0)
                    } else {
                        Color::new(1.0, 0.46, 0.42, 1.0)
                    },
                ),
            );
        }
    }
    draw_ui_text_ex(
        &format!(
            "Total spend: {} CR | Revenue: {} CR | Best profit: {} CR",
            ctx.session.stats.total_spend,
            ctx.session.stats.total_revenue,
            ctx.session.stats.best_profit
        ),
        rect.x + 26.0,
        rect.bottom() - 24.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
}
