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
        Rect::new(90.0, 34.0, 1100.0, 650.0),
        true,
    );
}

pub(super) fn draw_research_page(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_research_lab(
        ctx,
        mouse,
        actions,
        Rect::new(18.0, 120.0, 1244.0, 586.0),
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
    if close_button {
        draw_surface(
            rect,
            &SurfaceStyle::new(Color::new(0.018, 0.047, 0.068, 1.0))
                .with_border(2.0, Color::new(0.20, 0.63, 0.82, 1.0))
                .with_top_highlight(2.0, Color::new(0.36, 0.82, 1.0, 0.9)),
        );
    } else {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.012, 0.034, 0.052, 1.0),
        );
    }
    let title_y = rect.y + 40.0;
    draw_ui_text_ex(
        "RESEARCH LAB",
        rect.x + 24.0,
        title_y,
        TextStyle::new(13.0, Color::new(0.52, 0.76, 1.0, 1.0)).params(),
    );
    draw_ui_text_ex(
        "Turn frontier evidence into capability",
        rect.x + 24.0,
        title_y + 34.0,
        TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        "Strong contracts reveal fields. Fund one practical breakthrough at a time.",
        rect.x + 26.0,
        title_y + 57.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );

    draw_ui_text_ex(
        "RESEARCH",
        rect.right() - 290.0,
        title_y - 8.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} RP", ctx.session.research_points),
        rect.right() - 290.0,
        title_y + 18.0,
        TextStyle::new(19.0, Color::new(0.70, 0.80, 1.0, 1.0)).params(),
    );
    draw_line(
        rect.right() - 188.0,
        title_y - 10.0,
        rect.right() - 188.0,
        title_y + 27.0,
        1.0,
        Color::new(0.25, 0.42, 0.54, 0.44),
    );
    draw_ui_text_ex(
        "CAPITAL",
        rect.right() - 168.0,
        title_y - 8.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        &format!("{} CR", ctx.session.credits),
        rect.right() - 168.0,
        title_y + 18.0,
        TextStyle::new(19.0, Color::new(0.48, 1.0, 0.64, 1.0)).params(),
    );
    if close_button
        && button(
            Rect::new(rect.right() - 48.0, rect.y + 16.0, 32.0, 32.0),
            "X",
            true,
            ButtonTone::Muted,
            mouse,
        )
    {
        actions.push(UiAction::CloseResearch);
    }

    let branch_panel = Rect::new(rect.x + 24.0, rect.y + 122.0, 258.0, rect.h - 144.0);
    draw_rectangle(
        branch_panel.x - 12.0,
        branch_panel.y - 12.0,
        branch_panel.w + 24.0,
        branch_panel.h + 24.0,
        Color::new(0.025, 0.058, 0.076, 0.88),
    );
    draw_ui_text_ex(
        "KNOWN FIELDS",
        branch_panel.x,
        branch_panel.y + 14.0,
        TextStyle::new(10.0, Color::new(0.45, 0.82, 0.96, 1.0)).params(),
    );
    draw_branches(ctx, branch_panel, mouse, actions);

    let node_panel = Rect::new(
        branch_panel.right() + 34.0,
        branch_panel.y - 12.0,
        rect.right() - branch_panel.right() - 58.0,
        branch_panel.h + 24.0,
    );
    draw_line(
        node_panel.x - 18.0,
        node_panel.y,
        node_panel.x - 18.0,
        node_panel.bottom(),
        1.0,
        Color::new(0.20, 0.52, 0.62, 0.24),
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
        let row = Rect::new(panel.x, panel.y + 34.0 + index as f32 * 68.0, panel.w, 58.0);
        let active = ctx.research_branch == *key;
        if active {
            draw_rectangle(
                row.x - 8.0,
                row.y,
                row.w + 8.0,
                row.h,
                Color::new(0.10, 0.25, 0.34, 0.94),
            );
        }
        let accent = if discovered {
            Color::new(0.43, 0.84, 1.0, 1.0)
        } else {
            Color::new(0.30, 0.36, 0.39, 1.0)
        };
        draw_circle(
            row.x + 5.0,
            row.y + 20.0,
            if active { 6.0 } else { 4.0 },
            accent,
        );
        draw_ui_text_ex(
            if discovered {
                label
            } else {
                "UNDISCOVERED FIELD"
            },
            row.x + 20.0,
            row.y + 23.0,
            TextStyle::new(
                12.0,
                if discovered {
                    dark::TEXT_BRIGHT
                } else {
                    dark::TEXT_DIM
                },
            )
            .params(),
        );
        let detail = if discovered {
            format!("{} KN available", ctx.session.knowledge(key))
        } else {
            discovery_hint(key).to_owned()
        };
        draw_ui_text_ex(
            &detail,
            row.x + 20.0,
            row.y + 44.0,
            TextStyle::new(8.0, dark::TEXT_DIM).params(),
        );
        draw_line(
            row.x + 20.0,
            row.bottom(),
            row.right(),
            row.bottom(),
            1.0,
            Color::new(0.16, 0.36, 0.42, 0.32),
        );
        if discovered && row.contains_point(mouse) && is_mouse_button_released(MouseButton::Left) {
            actions.push(UiAction::SetResearchBranch((*key).to_owned()));
        }
    }
}

fn discovery_hint(key: &str) -> &'static str {
    match key {
        "hydrology" => "Close strong oceanic contracts",
        "volcanology" => "Close strong volcanic contracts",
        "atmospherics" => "Close deals with air specialists",
        "harsh-world" => "Close strong harsh-world contracts",
        "ecology" => "Close strong ecological contracts",
        _ => "Close specialist contracts",
    }
}

fn draw_nodes(ctx: &UiContext<'_>, panel: Rect, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let label = KNOWLEDGE_FIELDS
        .iter()
        .find(|(key, _)| *key == ctx.research_branch)
        .map(|(_, label)| *label)
        .unwrap_or("Research");
    draw_ui_text_ex(
        "ACTIVE DISCIPLINE",
        panel.x,
        panel.y + 20.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        label,
        panel.x,
        panel.y + 51.0,
        TextStyle::new(23.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} expertise available  /  choose the next capability",
            ctx.session.knowledge(ctx.research_branch)
        ),
        panel.x,
        panel.y + 73.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
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
        draw_ui_text_ex(
            "NO SIGNAL YET",
            panel.x + 44.0,
            panel.y + 210.0,
            TextStyle::new(20.0, dark::TEXT_DIM).params(),
        );
        draw_text_block(
            "Close stronger specialist deals to reveal this research direction.",
            panel.x + 44.0,
            panel.y + 228.0,
            panel.w - 88.0,
            70.0,
            12.0,
            4.0,
            dark::TEXT_DIM,
        );
        return;
    }
    let path_x = panel.x + 20.0;
    let first_y = panel.y + 119.0;
    let last_y = first_y + nodes.len().saturating_sub(1) as f32 * 112.0;
    draw_line(
        path_x,
        first_y,
        path_x,
        last_y,
        3.0,
        Color::new(0.24, 0.58, 0.76, 0.28),
    );
    for (index, node) in nodes.into_iter().enumerate() {
        let row = Rect::new(
            panel.x + 44.0,
            panel.y + 94.0 + index as f32 * 112.0,
            panel.w - 44.0,
            100.0,
        );
        draw_node_card(ctx, node, row, vec2(path_x, row.y + 25.0), mouse, actions);
    }
}

fn draw_node_card(
    ctx: &UiContext<'_>,
    node: &crate::data::ResearchDef,
    row: Rect,
    marker: Vec2,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let complete = ctx.session.research_is_complete(&node.name);
    let prerequisite = ctx.session.research_prerequisite_met(node);
    let affordable = ctx.session.research_points >= node.rp_cost.max(0)
        && ctx.session.credits >= node.credit_cost.max(0);
    let accent = if complete {
        Color::new(0.42, 1.0, 0.62, 1.0)
    } else if prerequisite {
        Color::new(0.47, 0.78, 1.0, 1.0)
    } else {
        Color::new(0.34, 0.40, 0.44, 1.0)
    };
    draw_circle(
        marker.x,
        marker.y,
        10.0,
        Color::new(accent.r, accent.g, accent.b, 0.16),
    );
    draw_circle(marker.x, marker.y, 5.0, accent);
    if complete {
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            Color::new(0.04, 0.15, 0.11, 0.64),
        );
    }
    draw_ui_text_ex(
        &format!("T{}  {}", node.tier.max(1), node.name),
        row.x,
        row.y + 24.0,
        TextStyle::new(15.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} RP  +  {} CR  /  {}",
            node.rp_cost.max(0),
            node.credit_cost.max(0),
            node.category
        ),
        row.x,
        row.y + 44.0,
        TextStyle::new(9.0, Color::new(0.68, 0.80, 0.92, 1.0)).params(),
    );
    draw_text_block(
        &node.description,
        row.x,
        row.y + 51.0,
        row.w - 166.0,
        35.0,
        9.0,
        2.0,
        dark::TEXT_DIM,
    );
    if let Some(reveal) = node.reveals.as_deref() {
        draw_ui_text_ex(
            &format!("REVEALS  {}", reveal),
            row.x,
            row.bottom() - 5.0,
            TextStyle::new(8.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
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
        Rect::new(row.right() - 142.0, row.y + 24.0, 130.0, 38.0),
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
    draw_line(
        row.x,
        row.bottom(),
        row.right(),
        row.bottom(),
        1.0,
        Color::new(0.18, 0.39, 0.44, 0.30),
    );
}
