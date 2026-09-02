use super::*;

pub(super) fn draw_tutorial(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    match ctx.session.tutorial_step {
        TutorialStep::Welcome => draw_welcome(mouse, actions),
        TutorialStep::BuyPlanet => draw_acquire_hint(),
        TutorialStep::ChooseOffer => draw_coach("STEP 2 / CHOOSE", "Tap an offer card to inspect its scan, then tap ACQUIRE WORLD.", Some(Rect::new(898.0, 602.0, 310.0, 52.0))),
        TutorialStep::SelectPlanet => draw_coach("STEP 3 / ACTIVATE", "Tap the purchased world in OWNED WORLDS to bring it into the engineering cradle.", Some(Rect::new(368.0, 616.0, 166.0, 45.0))),
        TutorialStep::InspectBuyer if ctx.expanded_buyer.is_some() => draw_coach("STEP 4 / READ DEMAND", "Review the six requirement rows, then tap HOLD / RETURN TO WORKSHOP to engineer this world.", Some(Rect::new(866.0, 654.0, 374.0, 32.0))),
        TutorialStep::InspectBuyer => draw_coach("STEP 4 / READ DEMAND", "Tap a buyer signal in LIVE NEGOTIATIONS. Green requirements already match; red ones need engineering.", Some(Rect::new(390.0, 240.0, 450.0, 92.0))),
        TutorialStep::UseTool => draw_coach("STEP 5 / ENGINEER", "Tap a tool in TERRAFORMING ARRAY, review OUTCOME CONSOLE, then tap APPLY TOOL.", Some(Rect::new(34.0, 266.0, 294.0, 58.0))),
        TutorialStep::SellOrSalvage => draw_coach("STEP 6 / CLOSE THE DEAL", "Tap CLOSE DEAL when at least 4 of 6 requirements match, or tap SALVAGE in Workshop to recover credits.", None),
        TutorialStep::OpenResearch => draw_coach("STEP 7 / GROW", "Sales award RP. Tap RESEARCH to inspect technology that opens new strategies.", Some(Rect::new(536.0, 74.0, 160.0, 34.0))),
        TutorialStep::Complete => {}
    }
}

fn draw_acquire_hint() {
    let focus = Rect::new(214.0, 613.0, 456.0, 62.0);
    draw_rectangle_lines(
        focus.x - 5.0,
        focus.y - 5.0,
        focus.w + 10.0,
        focus.h + 10.0,
        3.0,
        Color::new(0.42, 0.92, 1.0, 0.82),
    );
    let hint = Rect::new(252.0, 558.0, 380.0, 44.0);
    draw_surface(
        hint,
        &SurfaceStyle::new(Color::new(0.025, 0.09, 0.13, 0.98))
            .with_top_highlight(2.0, Color::new(0.42, 0.90, 1.0, 0.85)),
    );
    draw_ui_text_ex(
        "FIRST CONTRACT",
        hint.x + 14.0,
        hint.y + 17.0,
        TextStyle::new(9.0, Color::new(0.42, 0.88, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Tap the scan control below.",
        hint.x + 14.0,
        hint.y + 35.0,
        TextStyle::new(12.0, dark::TEXT_BRIGHT).params(),
    );
    draw_line(
        hint.center().x,
        hint.bottom(),
        hint.center().x,
        focus.y - 5.0,
        2.0,
        Color::new(0.42, 0.90, 1.0, 0.72),
    );
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
    let coach_y = focus
        .map(|target| {
            if target.center().y > 430.0 {
                152.0
            } else {
                626.0
            }
        })
        .unwrap_or(626.0);
    let rect = Rect::new(350.0, coach_y, 540.0, 76.0);
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
