use super::*;

pub(super) fn draw_acquisition(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let hero = Rect::new(18.0, 150.0, 1244.0, 176.0);
    draw_surface(
        hero,
        &SurfaceStyle::new(Color::new(0.045, 0.10, 0.145, 1.0))
            .with_border(1.0, Color::new(0.20, 0.55, 0.70, 1.0))
            .with_left_accent(6.0, Color::new(0.28, 0.82, 1.0, 1.0)),
    );
    draw_ui_text_ex(
        "FRONTIER ACQUISITION DESK",
        hero.x + 32.0,
        hero.y + 42.0,
        TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        "Scout difficult worlds before committing capital. Each catalogue scan presents a fresh set of planet contracts; compare the acquisition cost and starting environment against live alien demand before you buy.",
        hero.x + 34.0,
        hero.y + 66.0,
        700.0,
        70.0,
        15.0,
        4.0,
        Color::new(0.66, 0.80, 0.86, 1.0),
    );
    if button(
        Rect::new(hero.right() - 324.0, hero.y + 46.0, 276.0, 64.0),
        "SCAN FOR PLANET CONTRACTS",
        ctx.session.game_started,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenPurchase);
    }
    draw_ui_text_ex(
        "A scan is free. Credits are spent only when you accept a contract.",
        hero.right() - 330.0,
        hero.y + 135.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );

    draw_acquisition_metric(
        Rect::new(18.0, 342.0, 294.0, 150.0),
        "AVAILABLE CAPITAL",
        &format!("{} CR", ctx.session.credits),
        "Keep enough reserve for terraforming tools and recovery.",
    );
    draw_acquisition_metric(
        Rect::new(328.0, 342.0, 294.0, 150.0),
        "WORLDS IN PORTFOLIO",
        &ctx.session.planets.len().to_string(),
        "Select owned worlds from the Workshop before engineering.",
    );
    draw_acquisition_metric(
        Rect::new(638.0, 342.0, 294.0, 150.0),
        "CONTRACT REACH",
        &format!("{} OFFERS", contract_option_count(ctx.session.reputation)),
        "Reputation expands each frontier scan from three worlds to as many as five.",
    );
    draw_acquisition_metric(
        Rect::new(948.0, 342.0, 314.0, 150.0),
        "RECOVERY FLOOR",
        "25%",
        "Salvage returns part of total investment when a deal cannot be saved.",
    );

    let tip = Rect::new(18.0, 508.0, 1244.0, 182.0);
    draw_surface(
        tip,
        &SurfaceStyle::new(Color::new(0.035, 0.075, 0.10, 1.0))
            .with_border(1.0, Color::new(0.13, 0.32, 0.39, 1.0)),
    );
    draw_ui_text_ex(
        "CONTRACT BRIEFING",
        tip.x + 28.0,
        tip.y + 38.0,
        TextStyle::new(16.0, Color::new(0.45, 0.83, 0.96, 1.0)).params(),
    );
    draw_text_block(
        "1  Open the catalogue and compare acquisition costs.\n2  Check the Alien Market for valuable environmental targets.\n3  Buy a world with a plausible route to four of six matching requirements.\n4  Keep a credit reserve; salvage is a safety net, not a winning strategy.",
        tip.x + 30.0,
        tip.y + 62.0,
        780.0,
        94.0,
        14.0,
        5.0,
        Color::new(0.68, 0.80, 0.84, 1.0),
    );
    draw_text_block(
        "NEW COMPANY PRIORITY\nAcquire one world, inspect demand, then move to the Workshop. Your first sale funds research and establishes the company ledger.",
        tip.right() - 360.0,
        tip.y + 40.0,
        320.0,
        100.0,
        13.0,
        4.0,
        Color::new(0.42, 0.92, 0.66, 1.0),
    );
}

fn draw_acquisition_metric(rect: Rect, label: &str, value: &str, detail: &str) {
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.055, 0.105, 0.14, 1.0))
            .with_border(1.0, Color::new(0.17, 0.36, 0.43, 1.0)),
    );
    draw_ui_text_ex(
        label,
        rect.x + 18.0,
        rect.y + 27.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        value,
        rect.x + 18.0,
        rect.y + 72.0,
        TextStyle::new(31.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_block(
        detail,
        rect.x + 18.0,
        rect.y + 92.0,
        rect.w - 36.0,
        42.0,
        11.0,
        3.0,
        Color::new(0.52, 0.70, 0.76, 1.0),
    );
}
