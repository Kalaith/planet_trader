use super::*;

pub(super) fn draw_tutorial(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    match ctx.session.tutorial_step {
        TutorialStep::Welcome => draw_welcome(mouse, actions),
        TutorialStep::BuyPlanet => draw_coach("STEP 1 / ACQUIRE", "Tap SCAN FOR PLANET CONTRACTS to open the frontier catalogue.", Some(Rect::new(938.0, 196.0, 276.0, 64.0))),
        TutorialStep::ChooseOffer => draw_coach("STEP 2 / CHOOSE", "Compare the visible worlds, then tap a green BUY button.", None),
        TutorialStep::SelectPlanet => draw_coach("STEP 3 / ACTIVATE", "Tap the purchased world in PLANET INVENTORY to bring it into the workshop.", Some(Rect::new(344.0, 548.0, 572.0, 56.0))),
        TutorialStep::InspectBuyer => draw_coach("STEP 4 / READ DEMAND", "Tap an alien buyer card. Green requirements already match; red ones need engineering.", Some(Rect::new(950.0, 278.0, 304.0, 110.0))),
        TutorialStep::UseTool => draw_coach("STEP 5 / ENGINEER", "Tap a terraforming tool to inspect its cost and side effects, then tap USE.", Some(Rect::new(26.0, 218.0, 284.0, 78.0))),
        TutorialStep::SellOrSalvage => draw_coach("STEP 6 / CLOSE THE DEAL", "SELL when at least 4 of 6 requirements match. If the investment is poor, tap SALVAGE to recover credits.", None),
        TutorialStep::OpenResearch => draw_coach("STEP 7 / GROW", "Sales award RP. Tap RESEARCH to inspect technology that opens new strategies.", Some(Rect::new(536.0, 95.0, 160.0, 34.0))),
        TutorialStep::Complete => {}
    }
}

fn draw_welcome(mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.01, 0.03, 0.86),
    );
    let rect = Rect::new(260.0, 112.0, 760.0, 496.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.09, 0.13, 1.0))
            .with_border(2.0, Color::new(0.22, 0.76, 0.96, 1.0))
            .with_top_highlight(3.0, Color::new(0.45, 0.92, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "WELCOME, DIRECTOR",
        rect.x + 40.0,
        rect.y + 64.0,
        TextStyle::new(30.0, Color::new(0.50, 0.88, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Your first company charter has been approved.",
        rect.x + 42.0,
        rect.y + 96.0,
        TextStyle::new(16.0, dark::TEXT).params(),
    );
    draw_text_block("This orientation follows your real actions. It will stay with you across saves and point to the exact visible control needed next.\n\nYou will acquire a world, read alien demand, engineer its environment, close or recover a deal, and inspect the research earned by successful work.", rect.x + 44.0, rect.y + 138.0, rect.w - 88.0, 190.0, 16.0, 4.0, Color::new(0.68, 0.80, 0.84, 1.0));
    if button(
        Rect::new(rect.x + 126.0, rect.bottom() - 88.0, rect.w - 252.0, 54.0),
        "BEGIN GUIDED ORIENTATION",
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::BeginTutorial);
    }
}

fn draw_coach(kicker: &str, message: &str, focus: Option<Rect>) {
    if let Some(rect) = focus {
        draw_rectangle_lines(
            rect.x - 5.0,
            rect.y - 5.0,
            rect.w + 10.0,
            rect.h + 10.0,
            4.0,
            Color::new(0.16, 0.62, 0.78, 0.36),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.0,
            Color::new(0.42, 0.92, 1.0, 1.0),
        );
    }
    let rect = Rect::new(350.0, 626.0, 580.0, 76.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.025, 0.09, 0.13, 0.98))
            .with_border(2.0, Color::new(0.24, 0.76, 0.94, 1.0))
            .with_top_highlight(2.0, Color::new(0.42, 0.90, 1.0, 0.85)),
    );
    draw_ui_text_ex(
        kicker,
        rect.x + 18.0,
        rect.y + 24.0,
        TextStyle::new(12.0, Color::new(0.42, 0.88, 1.0, 1.0)).params(),
    );
    draw_text_block(
        message,
        rect.x + 18.0,
        rect.y + 33.0,
        rect.w - 36.0,
        38.0,
        12.5,
        2.0,
        dark::TEXT_BRIGHT,
    );
}
