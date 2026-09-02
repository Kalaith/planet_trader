use super::*;

#[path = "overlays/purchase.rs"]
mod purchase;

pub(super) fn draw_purchase_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    purchase::draw_purchase_modal(ctx, mouse, actions);
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
