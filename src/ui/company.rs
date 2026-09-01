use super::*;

pub(super) fn draw_company(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let stats = &ctx.session.stats;
    let realized = stats.total_profit;
    let (rank, next_rank) = company_rank(ctx.session.reputation);
    let rank_progress = if ctx.session.reputation >= 120 {
        format!("{}  //  highest company tier", rank)
    } else {
        format!(
            "{}  //  {} / {} reputation; the next tier expands contract reach",
            rank, ctx.session.reputation, next_rank
        )
    };
    draw_panel(
        Rect::new(18.0, 150.0, 1244.0, 540.0),
        "Company Charter & Ledger",
    );
    draw_ui_text_ex(
        &rank_progress,
        40.0,
        214.0,
        TextStyle::new(13.0, dark::TEXT_DIM).params(),
    );

    let metrics = [
        (
            "LIQUID CAPITAL",
            format!("{} CR", ctx.session.credits),
            "Available for contracts and tools",
        ),
        (
            "REALIZED PROFIT",
            format!("{}{realized} CR", if realized >= 0 { "+" } else { "" }),
            "Sales and salvage after investment",
        ),
        (
            "REPUTATION",
            format!("{} REP", ctx.session.reputation),
            "Earned from compatible, profitable sales",
        ),
        (
            "RESEARCH",
            format!("{} RP", ctx.session.research_points),
            "Spend in the Research Lab",
        ),
    ];
    for (index, (label, value, detail)) in metrics.iter().enumerate() {
        let card = Rect::new(40.0 + index as f32 * 296.0, 238.0, 278.0, 104.0);
        draw_surface(
            card,
            &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
                .with_border(1.0, Color::new(0.18, 0.39, 0.47, 1.0)),
        );
        draw_ui_text_ex(
            label,
            card.x + 14.0,
            card.y + 23.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            value,
            card.x + 14.0,
            card.y + 59.0,
            TextStyle::new(24.0, dark::TEXT_BRIGHT).params(),
        );
        draw_ui_text_ex(
            detail,
            card.x + 14.0,
            card.y + 84.0,
            TextStyle::new(10.0, Color::new(0.48, 0.68, 0.74, 1.0)).params(),
        );
    }

    let ledger = Rect::new(40.0, 362.0, 776.0, 304.0);
    draw_surface(
        ledger,
        &SurfaceStyle::new(Color::new(0.045, 0.09, 0.125, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "RECENT LEDGER",
        ledger.x + 16.0,
        ledger.y + 28.0,
        TextStyle::new(14.0, Color::new(0.46, 0.82, 0.96, 1.0)).params(),
    );
    if ctx.session.trade_history.is_empty() {
        draw_text_centered_in_box(
            "No contracts recorded. Acquire a world to begin the company ledger.",
            ledger.x + 24.0,
            ledger.y + 112.0,
            ledger.w - 48.0,
            50.0,
            14.0,
            dark::TEXT_DIM,
        );
    } else {
        for (row, trade) in ctx.session.trade_history.iter().rev().take(6).enumerate() {
            let y = ledger.y + 55.0 + row as f32 * 38.0;
            draw_ui_text_ex(
                &trade.transaction_type.to_uppercase(),
                ledger.x + 16.0,
                y,
                TextStyle::new(10.0, Color::new(0.42, 0.75, 0.84, 1.0)).params(),
            );
            draw_ui_text_ex(
                &trade.planet_name,
                ledger.x + 104.0,
                y,
                TextStyle::new(13.0, dark::TEXT).params(),
            );
            draw_text_right(
                &format!("{:+} CR", trade.profit),
                ledger.right() - 18.0,
                y,
                TextStyle::new(
                    12.0,
                    if trade.profit >= 0 {
                        Color::new(0.46, 0.94, 0.62, 1.0)
                    } else {
                        Color::new(1.0, 0.48, 0.44, 1.0)
                    },
                ),
            );
        }
    }

    let controls = Rect::new(834.0, 362.0, 406.0, 304.0);
    draw_surface(
        controls,
        &SurfaceStyle::new(Color::new(0.05, 0.10, 0.14, 1.0))
            .with_border(1.0, Color::new(0.16, 0.36, 0.44, 1.0)),
    );
    draw_ui_text_ex(
        "CHARTER CONTROLS",
        controls.x + 18.0,
        controls.y + 30.0,
        TextStyle::new(14.0, Color::new(0.46, 0.82, 0.96, 1.0)).params(),
    );
    if button(
        Rect::new(controls.x + 18.0, controls.y + 54.0, 176.0, 40.0),
        "SAVE NOW",
        true,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(controls.x + 212.0, controls.y + 54.0, 176.0, 40.0),
        "LOAD SAVE",
        ctx.save_exists,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::Load);
    }
    if button(
        Rect::new(controls.x + 18.0, controls.y + 112.0, 370.0, 40.0),
        "RESTART ORIENTATION",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::RestartTutorial);
    }
    if button(
        Rect::new(controls.x + 18.0, controls.y + 170.0, 370.0, 40.0),
        "RESET COMPANY",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::OpenResetConfirm);
    }
    draw_text_block("Autosave runs after purchases, tools, sales, research, market refreshes, and tutorial progress.", controls.x + 20.0, controls.y + 232.0, controls.w - 40.0, 46.0, 11.0, 3.0, dark::TEXT_DIM);
}
