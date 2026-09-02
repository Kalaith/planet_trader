use super::*;

pub(super) fn draw_research_modal(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_rectangle(
        0.0,
        0.0,
        LOGICAL_WIDTH,
        LOGICAL_HEIGHT,
        Color::new(0.0, 0.015, 0.03, 0.86),
    );
    draw_research_lab(
        ctx,
        mouse,
        actions,
        Rect::new(110.0, 34.0, 1060.0, 650.0),
        true,
    );
}

pub(super) fn draw_research_page(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_research_lab(
        ctx,
        mouse,
        actions,
        Rect::new(18.0, 150.0, 1244.0, 540.0),
        false,
    );
}

fn draw_research_lab(
    ctx: &UiContext<'_>,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
    rect: Rect,
    close_button: bool,
) {
    draw_surface(
        rect,
        &SurfaceStyle::new(Color::new(0.035, 0.075, 0.11, 1.0))
            .with_border(2.0, Color::new(0.20, 0.63, 0.82, 1.0))
            .with_top_highlight(3.0, Color::new(0.36, 0.82, 1.0, 0.9)),
    );
    draw_ui_text_ex(
        "RESEARCH & DISCOVERY",
        rect.x + 24.0,
        rect.y + 35.0,
        TextStyle::new(23.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Strong 5/6 and perfect 6/6 contracts reveal fields. RP buys understanding; credits build the capability.",
        rect.x + 26.0,
        rect.y + 58.0,
        TextStyle::new(11.0, dark::TEXT_DIM).params(),
    );
    draw_badge(
        Rect::new(rect.right() - 256.0, rect.y + 18.0, 180.0, 36.0),
        &format!(
            "{} RP  /  {} CR",
            ctx.session.research_points, ctx.session.credits
        ),
        Color::new(0.12, 0.17, 0.30, 1.0),
        Color::new(0.70, 0.80, 1.0, 1.0),
    );
    if close_button
        && button(
            Rect::new(rect.right() - 52.0, rect.y + 16.0, 34.0, 34.0),
            "X",
            true,
            ButtonTone::Muted,
            mouse,
        )
    {
        actions.push(UiAction::CloseResearch);
    }

    let branch_panel = Rect::new(rect.x + 20.0, rect.y + 78.0, 270.0, rect.h - 98.0);
    draw_surface(
        branch_panel,
        &SurfaceStyle::new(Color::new(0.045, 0.095, 0.13, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "KNOWN FIELDS",
        branch_panel.x + 16.0,
        branch_panel.y + 28.0,
        TextStyle::new(13.0, Color::new(0.45, 0.82, 0.96, 1.0)).params(),
    );
    draw_branches(ctx, branch_panel, mouse, actions);

    let node_panel = Rect::new(
        branch_panel.right() + 16.0,
        branch_panel.y,
        rect.right() - branch_panel.right() - 36.0,
        branch_panel.h,
    );
    draw_surface(
        node_panel,
        &SurfaceStyle::new(Color::new(0.045, 0.09, 0.125, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_nodes(ctx, node_panel, mouse, actions);
}

fn draw_branches(ctx: &UiContext<'_>, panel: Rect, mouse: Vec2, actions: &mut Vec<UiAction>) {
    for (index, (key, label)) in KNOWLEDGE_FIELDS.iter().enumerate() {
        let discovered = *key == "frontier"
            || ctx
                .data
                .research
                .iter()
                .any(|node| node.branch == *key && ctx.session.research_is_discovered(node));
        let card = Rect::new(
            panel.x + 12.0,
            panel.y + 42.0 + index as f32 * 64.0,
            panel.w - 24.0,
            54.0,
        );
        let active = ctx.research_branch == *key;
        let text = if discovered {
            format!("{}  //  {} KN", label, ctx.session.knowledge(key))
        } else {
            format!(
                "UNKNOWN  //  {}",
                match *key {
                    "hydrology" => "OCEANIC SALES",
                    "volcanology" => "VOLCANIC SALES",
                    "atmospherics" => "AIR SPECIALISTS",
                    "harsh-world" => "HARSH-WORLD SALES",
                    "ecology" => "ECOLOGICAL SALES",
                    _ => "SPECIALIST SALES",
                }
            )
        };
        if button(
            card,
            &text,
            discovered,
            if active {
                ButtonTone::Primary
            } else {
                ButtonTone::Muted
            },
            mouse,
        ) {
            actions.push(UiAction::SetResearchBranch((*key).to_owned()));
        }
    }
}

fn draw_nodes(ctx: &UiContext<'_>, panel: Rect, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let label = KNOWLEDGE_FIELDS
        .iter()
        .find(|(key, _)| *key == ctx.research_branch)
        .map(|(_, label)| *label)
        .unwrap_or("Research");
    draw_ui_text_ex(
        label,
        panel.x + 18.0,
        panel.y + 30.0,
        TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "Expertise {}  //  select one capability to fund",
            ctx.session.knowledge(ctx.research_branch)
        ),
        panel.x + 18.0,
        panel.y + 50.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    let nodes: Vec<_> = ctx
        .data
        .research
        .iter()
        .filter(|node| {
            node.branch == ctx.research_branch && ctx.session.research_is_discovered(node)
        })
        .collect();
    if nodes.is_empty() {
        draw_text_centered_in_box(
            "This direction is still unknown. Close stronger specialist deals to reveal it.",
            panel.x + 70.0,
            panel.y + 190.0,
            panel.w - 140.0,
            70.0,
            15.0,
            dark::TEXT_DIM,
        );
        return;
    }
    for (index, node) in nodes.into_iter().enumerate() {
        let card = Rect::new(
            panel.x + 16.0,
            panel.y + 68.0 + index as f32 * 100.0,
            panel.w - 32.0,
            90.0,
        );
        draw_node_card(ctx, node, card, mouse, actions);
    }
}

fn draw_node_card(
    ctx: &UiContext<'_>,
    node: &crate::data::ResearchDef,
    card: Rect,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let complete = ctx.session.research_is_complete(&node.name);
    let prerequisite = ctx.session.research_prerequisite_met(node);
    let affordable = ctx.session.research_points >= node.rp_cost.max(0)
        && ctx.session.credits >= node.credit_cost.max(0);
    draw_surface(
        card,
        &SurfaceStyle::new(if complete {
            Color::new(0.06, 0.22, 0.15, 1.0)
        } else {
            Color::new(0.07, 0.135, 0.18, 1.0)
        })
        .with_border(
            1.0,
            if complete {
                Color::new(0.30, 0.76, 0.46, 1.0)
            } else {
                Color::new(0.22, 0.42, 0.50, 1.0)
            },
        ),
    );
    draw_ui_text_ex(
        &format!("T{}  {}", node.tier.max(1), node.name),
        card.x + 12.0,
        card.y + 21.0,
        TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} RP  +  {} CR  //  {}",
            node.rp_cost.max(0),
            node.credit_cost.max(0),
            node.category
        ),
        card.x + 12.0,
        card.y + 40.0,
        TextStyle::new(10.0, Color::new(0.68, 0.80, 0.92, 1.0)).params(),
    );
    draw_text_block(
        &node.description,
        card.x + 12.0,
        card.y + 48.0,
        card.w - 160.0,
        30.0,
        10.0,
        2.0,
        dark::TEXT_DIM,
    );
    if let Some(reveal) = node.reveals.as_deref() {
        draw_ui_text_ex(
            &format!("REVEALS: {}", reveal),
            card.x + 12.0,
            card.bottom() - 8.0,
            TextStyle::new(9.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
        );
    }
    let enabled = !complete && prerequisite && affordable;
    let label = if complete {
        "COMPLETE"
    } else if !prerequisite {
        "PREREQUISITE"
    } else if !affordable {
        "NEED FUNDS"
    } else {
        "RESEARCH"
    };
    if button(
        Rect::new(card.right() - 140.0, card.y + 24.0, 124.0, 38.0),
        label,
        enabled,
        if complete {
            ButtonTone::Positive
        } else {
            ButtonTone::Primary
        },
        mouse,
    ) {
        actions.push(UiAction::CompleteResearch(node.name.clone()));
    }
}
