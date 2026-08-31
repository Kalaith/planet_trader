use super::*;

pub(super) fn draw_research_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.015, 0.03, 0.86),
    );

    let rect = Rect::new(170.0, 42.0, 940.0, 636.0);
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.045, 0.09, 0.13, 1.0))
            .with_border(2.0, Color::new(0.22, 0.70, 0.92, 1.0))
            .with_top_highlight(3.0, Color::new(0.40, 0.86, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "Research Lab",
        rect.x + 24.0,
        rect.y + 40.0,
        TextStyle::new(24.0, Color::new(0.48, 0.84, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Invest RP earned from successful sales to unlock advanced terraforming tools.",
        rect.x + 26.0,
        rect.y + 64.0,
        TextStyle::new(12.0, dark::TEXT_DIM).params(),
    );
    draw_badge(
        Rect::new(rect.right() - 198.0, rect.y + 20.0, 132.0, 34.0),
        &format!("Research: {} RP", ctx.session.research_points),
        Color::new(0.14, 0.19, 0.34, 1.0),
        Color::new(0.66, 0.76, 1.0, 1.0),
    );
    if button(
        Rect::new(rect.right() - 54.0, rect.y + 16.0, 34.0, 34.0),
        "X",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::CloseResearch);
    }

    if ctx.data.research.is_empty() {
        draw_text_centered_in_box(
            "No research nodes are available.",
            rect.x + 24.0,
            rect.y + 250.0,
            rect.w - 48.0,
            32.0,
            17.0,
            dark::TEXT_DIM,
        );
        return;
    }

    for (index, research) in ctx.data.research.iter().enumerate() {
        let column = index % 2;
        let row = index / 2;
        let card = Rect::new(
            rect.x + 24.0 + column as f32 * 454.0,
            rect.y + 88.0 + row as f32 * 104.0,
            438.0,
            94.0,
        );
        draw_research_card(ctx, research, card, mouse, actions);
    }

    draw_ui_text_ex(
        "Higher compatibility and sale prices award more RP.",
        rect.x + 26.0,
        rect.bottom() - 20.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
}

fn draw_research_card(
    ctx: &UiContext<'_>,
    research: &crate::data::ResearchDef,
    card: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let complete = ctx.session.research_is_complete(&research.name);
    let affordable = ctx.session.research_points >= research.rp_cost.max(0);
    let hovered = card.contains_point(mouse);
    let fill = if complete {
        Color::new(0.08, 0.24, 0.17, 1.0)
    } else if hovered {
        Color::new(0.11, 0.20, 0.28, 1.0)
    } else {
        Color::new(0.07, 0.13, 0.18, 1.0)
    };
    draw_surface(
        card,
        &SurfaceStyle::new(fill).with_border(
            1.0,
            if complete {
                Color::new(0.30, 0.72, 0.44, 1.0)
            } else {
                Color::new(0.24, 0.42, 0.50, 1.0)
            },
        ),
    );
    draw_ui_text_ex(
        &research.name,
        card.x + 12.0,
        card.y + 21.0,
        TextStyle::new(
            14.0,
            if complete {
                dark::TEXT_BRIGHT
            } else {
                dark::TEXT
            },
        )
        .params(),
    );
    draw_ui_text_ex(
        &format!("{} | {} RP", research.category, research.rp_cost.max(0)),
        card.x + 12.0,
        card.y + 39.0,
        TextStyle::new(10.0, Color::new(0.57, 0.76, 0.82, 1.0)).params(),
    );
    draw_text_block(
        &research.description,
        card.x + 12.0,
        card.y + 50.0,
        card.w - 132.0,
        30.0,
        10.0,
        2.0,
        dark::TEXT_DIM,
    );

    let enabled = !complete && affordable;
    let label = if complete {
        "DONE"
    } else if affordable {
        "RESEARCH"
    } else {
        "NEED RP"
    };
    if button(
        Rect::new(card.right() - 108.0, card.y + 16.0, 96.0, 32.0),
        label,
        enabled,
        if complete {
            ButtonTone::Positive
        } else {
            ButtonTone::Primary
        },
        mouse,
    ) {
        actions.push(UiAction::CompleteResearch(research.name.clone()));
    }
}
