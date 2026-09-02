use super::*;

pub(super) fn draw_company(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_company_floor();
    let stats = &ctx.session.stats;
    let (rank, next_rank) = company_rank(ctx.session.reputation);

    draw_ui_text_ex(
        "COMPANY COMMAND",
        42.0,
        162.0,
        TextStyle::new(13.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
    );
    draw_ui_text_ex(
        rank,
        40.0,
        203.0,
        TextStyle::new(30.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        if ctx.session.reputation >= 120 {
            "Highest brokerage tier achieved"
        } else {
            "Build reputation through compatible, profitable contracts"
        },
        42.0,
        228.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    draw_rank_progress(ctx.session.reputation, next_rank);

    let metrics = [
        (
            "LIQUID CAPITAL",
            format!("{} CR", ctx.session.credits),
            Color::new(0.48, 1.0, 0.64, 1.0),
        ),
        (
            "REALIZED PROFIT",
            format!(
                "{}{} CR",
                if stats.total_profit >= 0 { "+" } else { "" },
                stats.total_profit
            ),
            if stats.total_profit >= 0 {
                Color::new(0.48, 1.0, 0.64, 1.0)
            } else {
                Color::new(1.0, 0.52, 0.48, 1.0)
            },
        ),
        (
            "REPUTATION",
            format!("{} REP", ctx.session.reputation),
            Color::new(1.0, 0.78, 0.38, 1.0),
        ),
        (
            "RESEARCH",
            format!("{} RP", ctx.session.research_points),
            Color::new(0.70, 0.80, 1.0, 1.0),
        ),
    ];
    for (index, (label, value, color)) in metrics.iter().enumerate() {
        let x = 42.0 + index as f32 * 294.0;
        if index > 0 {
            draw_line(
                x - 22.0,
                270.0,
                x - 22.0,
                333.0,
                1.0,
                Color::new(0.22, 0.42, 0.46, 0.36),
            );
        }
        draw_ui_text_ex(
            label,
            x,
            280.0,
            TextStyle::new(8.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(value, x, 317.0, TextStyle::new(23.0, *color).params());
    }
    draw_line(
        40.0,
        346.0,
        1240.0,
        346.0,
        1.0,
        Color::new(0.22, 0.49, 0.54, 0.34),
    );

    draw_ledger(ctx);
    draw_knowledge(ctx);
    draw_company_controls(mouse, actions);
}

fn draw_company_floor() {
    draw_rectangle(
        18.0,
        120.0,
        1244.0,
        586.0,
        Color::new(0.013, 0.035, 0.051, 1.0),
    );
    draw_rectangle(
        812.0,
        352.0,
        450.0,
        354.0,
        Color::new(0.025, 0.058, 0.073, 0.88),
    );
    draw_line(
        806.0,
        368.0,
        806.0,
        686.0,
        1.0,
        Color::new(0.20, 0.52, 0.60, 0.24),
    );
}

fn draw_rank_progress(reputation: i64, next_rank: i64) {
    let start = if reputation >= 120 {
        120
    } else if reputation >= 60 {
        60
    } else if reputation >= 25 {
        25
    } else {
        0
    };
    let progress = if reputation >= 120 {
        1.0
    } else {
        (reputation.saturating_sub(start)) as f32 / next_rank.saturating_sub(start).max(1) as f32
    };
    let bar = Rect::new(742.0, 185.0, 476.0, 8.0);
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w,
        bar.h,
        Color::new(0.08, 0.16, 0.19, 1.0),
    );
    draw_rectangle(
        bar.x,
        bar.y,
        bar.w * progress.clamp(0.0, 1.0),
        bar.h,
        Color::new(0.28, 0.80, 0.90, 1.0),
    );
    draw_ui_text_ex(
        "BROKERAGE STANDING",
        bar.x,
        171.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    let standing = if reputation >= 120 {
        "MAXIMUM".to_owned()
    } else {
        format!("{} / {} REP", reputation, next_rank)
    };
    draw_text_right(
        &standing,
        bar.right(),
        171.0,
        TextStyle::new(9.0, Color::new(0.55, 0.83, 0.90, 1.0)),
    );
    draw_ui_text_ex(
        "Higher standing expands frontier contract reach.",
        bar.x,
        217.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
}

fn draw_ledger(ctx: &UiContext<'_>) {
    let stats = &ctx.session.stats;
    draw_ui_text_ex(
        "RECENT CONTRACTS",
        42.0,
        380.0,
        TextStyle::new(13.0, Color::new(0.46, 0.82, 0.96, 1.0)).params(),
    );
    draw_ui_text_ex(
        "The company record, newest first",
        42.0,
        402.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
    if ctx.session.trade_history.is_empty() {
        draw_ui_text_ex(
            "NO CONTRACTS RECORDED",
            42.0,
            459.0,
            TextStyle::new(20.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            "Acquire a world to begin the company ledger.",
            42.0,
            486.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
    } else {
        for (row, trade) in ctx.session.trade_history.iter().rev().take(4).enumerate() {
            let y = 438.0 + row as f32 * 42.0;
            draw_ui_text_ex(
                &trade.transaction_type.to_uppercase(),
                42.0,
                y,
                TextStyle::new(8.0, Color::new(0.42, 0.75, 0.84, 1.0)).params(),
            );
            draw_ui_text_ex(
                &trade.planet_name,
                118.0,
                y,
                TextStyle::new(13.0, dark::TEXT).params(),
            );
            draw_text_right(
                &format!("{:+} CR", trade.profit),
                760.0,
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
            draw_line(
                42.0,
                y + 12.0,
                760.0,
                y + 12.0,
                1.0,
                Color::new(0.16, 0.36, 0.41, 0.28),
            );
        }
    }
    draw_ui_text_ex(
        "LIFETIME OPERATIONS",
        42.0,
        616.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    let operations = [
        ("WORLDS ACQUIRED", stats.planets_purchased),
        ("DEALS CLOSED", stats.planets_sold),
        ("SALVAGED", stats.planets_salvaged),
    ];
    for (index, (label, value)) in operations.iter().enumerate() {
        let x = 42.0 + index as f32 * 170.0;
        draw_ui_text_ex(
            &value.to_string(),
            x,
            652.0,
            TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
        );
        draw_ui_text_ex(
            label,
            x,
            674.0,
            TextStyle::new(8.0, dark::TEXT_DIM).params(),
        );
    }
    draw_ui_text_ex(
        &format!("{:+} CR", stats.best_profit),
        570.0,
        652.0,
        TextStyle::new(20.0, Color::new(0.48, 1.0, 0.64, 1.0)).params(),
    );
    draw_ui_text_ex(
        "BEST CONTRACT",
        570.0,
        674.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
}

fn draw_knowledge(ctx: &UiContext<'_>) {
    draw_ui_text_ex(
        "SPECIES KNOWLEDGE",
        836.0,
        380.0,
        TextStyle::new(13.0, Color::new(0.46, 0.82, 0.96, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Expertise earned from strong specialist sales",
        836.0,
        402.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
    for (index, (key, label)) in KNOWLEDGE_FIELDS
        .iter()
        .filter(|(key, _)| *key != "frontier")
        .enumerate()
    {
        let value = ctx.session.knowledge(key);
        let next = ctx
            .data
            .research
            .iter()
            .filter(|node| node.branch == *key && node.knowledge_required > value)
            .map(|node| node.knowledge_required)
            .min();
        let y = 438.0 + index as f32 * 39.0;
        draw_ui_text_ex(label, 836.0, y, TextStyle::new(10.0, dark::TEXT).params());
        draw_text_right(
            &next
                .map(|target| format!("{} / {} KN", value, target))
                .unwrap_or_else(|| format!("{} KN  /  MASTERED", value)),
            1228.0,
            y,
            TextStyle::new(
                9.0,
                if value >= 2 {
                    Color::new(0.46, 0.94, 0.62, 1.0)
                } else {
                    dark::TEXT_DIM
                },
            ),
        );
        let width = next
            .map(|target| value as f32 / target.max(1) as f32)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0)
            * 392.0;
        draw_rectangle(
            836.0,
            y + 10.0,
            392.0,
            3.0,
            Color::new(0.09, 0.18, 0.21, 1.0),
        );
        draw_rectangle(
            836.0,
            y + 10.0,
            width,
            3.0,
            Color::new(0.25, 0.68, 0.75, 0.78),
        );
    }
}

fn draw_company_controls(mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "CHARTER CONTROLS",
        836.0,
        645.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(836.0, 658.0, 108.0, 32.0),
        "SAVE",
        true,
        ButtonTone::Positive,
        mouse,
    ) {
        actions.push(UiAction::Save);
    }
    if button(
        Rect::new(952.0, 658.0, 128.0, 32.0),
        "ORIENTATION",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::RestartTutorial);
    }
    if button(
        Rect::new(1088.0, 658.0, 140.0, 32.0),
        "RESET COMPANY",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::OpenResetConfirm);
    }
}
