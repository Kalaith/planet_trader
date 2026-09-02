use super::*;

const TOOL_RAIL: Rect = Rect::new(34.0, 140.0, 294.0, 548.0);
const PLANET_STAGE: Rect = Rect::new(348.0, 120.0, 574.0, 586.0);
const INSPECTOR: Rect = Rect::new(940.0, 140.0, 300.0, 548.0);

pub(super) fn draw_workshop(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_workshop_floor();
    draw_tool_rail(ctx, mouse, actions);
    draw_planet_stage(ctx, mouse, actions);
    draw_inspector(ctx, mouse, actions);
}

fn draw_workshop_floor() {
    draw_rectangle(
        18.0,
        120.0,
        1244.0,
        586.0,
        Color::new(0.012, 0.035, 0.052, 1.0),
    );
    draw_rectangle(
        18.0,
        120.0,
        326.0,
        586.0,
        Color::new(0.022, 0.055, 0.073, 0.96),
    );
    draw_rectangle(
        928.0,
        120.0,
        334.0,
        586.0,
        Color::new(0.025, 0.058, 0.075, 0.96),
    );
    draw_line(
        344.0,
        142.0,
        344.0,
        684.0,
        1.0,
        Color::new(0.20, 0.56, 0.66, 0.22),
    );
    draw_line(
        928.0,
        142.0,
        928.0,
        684.0,
        1.0,
        Color::new(0.20, 0.56, 0.66, 0.22),
    );
    for index in 0..36 {
        let value = index as f32;
        let x = 370.0 + (value * 113.7) % 530.0;
        let y = 145.0 + (value * 67.3) % 410.0;
        draw_circle(
            x,
            y,
            0.8,
            Color::new(0.32, 0.72, 0.82, 0.10 + (value.sin().abs() * 0.18)),
        );
    }
}

fn draw_tool_rail(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "TERRAFORMING ARRAY",
        TOOL_RAIL.x,
        164.0,
        TextStyle::new(14.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
    );
    let unlocked: Vec<_> = ctx
        .data
        .terraforming_tools
        .iter()
        .filter(|tool| !tool_is_locked(tool, &ctx.session.completed_research))
        .collect();
    draw_ui_text_ex(
        &format!("{} tools online", unlocked.len()),
        TOOL_RAIL.x,
        187.0,
        TextStyle::new(9.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(224.0, 145.0, 94.0, 30.0),
        "RESEARCH",
        true,
        ButtonTone::Muted,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Research));
    }

    let variable = analysis_level(&ctx.session.completed_research) >= 2;
    for (index, intensity) in [
        ToolIntensity::Low,
        ToolIntensity::Standard,
        ToolIntensity::Heavy,
    ]
    .into_iter()
    .enumerate()
    {
        let enabled = intensity == ToolIntensity::Standard || variable;
        if button(
            Rect::new(34.0 + index as f32 * 98.0, 205.0, 90.0, 30.0),
            intensity.label(),
            enabled,
            if ctx.tool_intensity == intensity {
                ButtonTone::Primary
            } else {
                ButtonTone::Muted
            },
            mouse,
        ) {
            actions.push(UiAction::SetToolIntensity(intensity));
        }
    }
    if !variable {
        draw_ui_text_ex(
            "LOW / HEAVY require Systems Modeling",
            34.0,
            251.0,
            TextStyle::new(8.0, Color::new(0.76, 0.61, 0.32, 1.0)).params(),
        );
    }

    let list = Rect::new(TOOL_RAIL.x, 266.0, TOOL_RAIL.w, 414.0);
    let total: f32 = unlocked
        .iter()
        .map(|tool| {
            if ctx.expanded_tool == Some(tool.id.as_str()) {
                134.0
            } else {
                58.0
            }
        })
        .sum::<f32>()
        + unlocked.len().saturating_sub(1) as f32 * 4.0;
    let max_scroll = (total - list.h).max(0.0);
    let scroll = ctx.tool_scroll.min(max_scroll);
    let mut y = list.y - scroll;
    for tool in unlocked {
        let selected = ctx.expanded_tool == Some(tool.id.as_str());
        let height = if selected { 134.0 } else { 58.0 };
        let row = Rect::new(list.x, y, list.w, height);
        if row.bottom() >= list.y && row.y <= list.bottom() {
            draw_tool_choice(ctx, tool, row, selected, mouse, actions);
        }
        y += height + 4.0;
    }
    if max_scroll > 0.0 {
        if button(
            Rect::new(246.0, 242.0, 32.0, 22.0),
            "UP",
            scroll > 0.0,
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::ScrollTools(-1));
        }
        if button(
            Rect::new(284.0, 242.0, 32.0, 22.0),
            "DN",
            scroll < max_scroll,
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::ScrollTools(1));
        }
    }
}

fn draw_tool_choice(
    ctx: &UiContext<'_>,
    tool: &Tool,
    row: Rect,
    selected: bool,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let hovered = row.contains_point(mouse);
    if selected || hovered {
        draw_rectangle(
            row.x,
            row.y,
            row.w,
            row.h,
            if selected {
                Color::new(0.07, 0.22, 0.28, 0.94)
            } else {
                Color::new(0.06, 0.13, 0.17, 0.80)
            },
        );
    }
    draw_rectangle(
        row.x,
        row.y + 7.0,
        3.0,
        row.h - 14.0,
        if selected {
            Color::new(0.36, 0.91, 1.0, 1.0)
        } else {
            Color::new(0.20, 0.55, 0.64, 0.66)
        },
    );
    draw_ui_text_ex(
        &format!("{}  {}", tool_icon(&tool.category), tool.name),
        row.x + 14.0,
        row.y + 22.0,
        TextStyle::new(13.0, dark::TEXT_BRIGHT).params(),
    );
    draw_text_right(
        &format!("{} CR", tool_cost(tool, ctx.tool_intensity)),
        row.right() - 8.0,
        row.y + 22.0,
        TextStyle::new(10.0, Color::new(0.64, 0.82, 0.86, 1.0)),
    );
    draw_ui_text_ex(
        ctx.tool_intensity.label(),
        row.x + 14.0,
        row.y + 43.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    if selected {
        draw_text_block(
            &tool.description,
            row.x + 14.0,
            row.y + 57.0,
            row.w - 28.0,
            32.0,
            9.0,
            2.0,
            dark::TEXT_DIM,
        );
        draw_ui_text_ex(
            "PROJECTED EFFECT",
            row.x + 14.0,
            row.y + 102.0,
            TextStyle::new(8.0, Color::new(0.40, 0.78, 0.86, 1.0)).params(),
        );
        draw_text_block(
            &effect_forecast(
                &tool.effect,
                ctx.tool_intensity,
                analysis_level(&ctx.session.completed_research),
            ),
            row.x + 14.0,
            row.y + 108.0,
            row.w - 28.0,
            24.0,
            9.0,
            1.0,
            Color::new(0.56, 0.88, 0.92, 1.0),
        );
    }
    draw_line(
        row.x + 10.0,
        row.bottom(),
        row.right(),
        row.bottom(),
        1.0,
        Color::new(0.15, 0.34, 0.40, 0.34),
    );
    if hovered && is_mouse_button_released(MouseButton::Left) {
        actions.push(UiAction::ToggleTool(tool.id.clone()));
    }
}

fn draw_planet_stage(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "ENGINEERING CRADLE",
        PLANET_STAGE.x + 22.0,
        164.0,
        TextStyle::new(13.0, Color::new(0.38, 0.76, 0.86, 1.0)).params(),
    );
    let Some(planet) = ctx.session.current_planet() else {
        draw_empty_cradle(mouse, actions);
        draw_portfolio(ctx, mouse, actions);
        return;
    };
    draw_ui_text_ex(
        &planet.name,
        PLANET_STAGE.x + 20.0,
        199.0,
        TextStyle::new(25.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{}  /  {} CR invested",
            planet.planet_type.name, planet.invested_cost
        ),
        PLANET_STAGE.x + 22.0,
        222.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );

    let center = vec2(PLANET_STAGE.center().x, 383.0);
    for radius in [180.0, 198.0, 216.0] {
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            1.0,
            Color::new(0.22, 0.70, 0.80, 0.12),
        );
    }
    draw_line(
        center.x - 230.0,
        center.y,
        center.x + 230.0,
        center.y,
        1.0,
        Color::new(0.24, 0.70, 0.78, 0.10),
    );
    draw_planet_orb(planet, center, 154.0);
    if let Some((buyer, score, _, profit)) = best_market_route(planet, &ctx.session.alien_buyers) {
        let matches = compatibility_matches(score);
        draw_ui_text_ex(
            "STRONGEST ROUTE",
            PLANET_STAGE.x + 22.0,
            535.0,
            TextStyle::new(8.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            &buyer.name,
            PLANET_STAGE.x + 22.0,
            558.0,
            TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_right(
            &format!("{}/6 MATCH  /  {:+} CR", matches, profit),
            PLANET_STAGE.right() - 22.0,
            558.0,
            TextStyle::new(
                12.0,
                if matches >= 4 {
                    Color::new(0.48, 1.0, 0.64, 1.0)
                } else {
                    Color::new(1.0, 0.76, 0.38, 1.0)
                },
            ),
        );
        draw_line(
            PLANET_STAGE.x + 22.0,
            569.0,
            PLANET_STAGE.right() - 22.0,
            569.0,
            1.0,
            Color::new(0.24, 0.58, 0.64, 0.32),
        );
    }
    draw_portfolio(ctx, mouse, actions);
}

fn draw_empty_cradle(mouse: Vec2, actions: &mut Vec<UiAction>) {
    let center = vec2(PLANET_STAGE.center().x, 374.0);
    for radius in [92.0, 132.0, 172.0] {
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            2.0,
            Color::new(0.20, 0.64, 0.75, 0.16),
        );
    }
    draw_ui_text_ex(
        "CRADLE EMPTY",
        center.x - 74.0,
        center.y - 4.0,
        TextStyle::new(21.0, dark::TEXT_DIM).params(),
    );
    draw_ui_text_ex(
        "Acquire a world to begin engineering.",
        center.x - 120.0,
        center.y + 25.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(center.x - 120.0, 466.0, 240.0, 48.0),
        "SCAN CONTRACTS",
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::OpenPurchase);
    }
}

fn draw_portfolio(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "OWNED WORLDS",
        PLANET_STAGE.x + 22.0,
        602.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    if ctx.session.planets.is_empty() {
        draw_ui_text_ex(
            "No worlds in the company portfolio",
            PLANET_STAGE.x + 22.0,
            635.0,
            TextStyle::new(11.0, dark::TEXT_DIM).params(),
        );
        return;
    }
    let start = ctx
        .inventory_scroll
        .min(ctx.session.planets.len().saturating_sub(1));
    for (index, planet) in ctx.session.planets.iter().skip(start).take(3).enumerate() {
        let selected = ctx.session.current_planet_id.as_deref() == Some(planet.id.as_str());
        if button(
            Rect::new(
                PLANET_STAGE.x + 20.0 + index as f32 * 176.0,
                616.0,
                166.0,
                45.0,
            ),
            &planet.name,
            true,
            if selected {
                ButtonTone::Primary
            } else {
                ButtonTone::Muted
            },
            mouse,
        ) {
            actions.push(UiAction::SelectPlanet(planet.id.clone()));
        }
    }
}

fn draw_inspector(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_ui_text_ex(
        "OUTCOME CONSOLE",
        INSPECTOR.x,
        164.0,
        TextStyle::new(14.0, Color::new(0.42, 0.82, 0.94, 1.0)).params(),
    );
    let Some(planet) = ctx.session.current_planet() else {
        draw_text_block("Select an owned world to reveal its environmental systems and preview an intervention.", INSPECTOR.x, 205.0, INSPECTOR.w, 80.0, 12.0, 4.0, dark::TEXT_DIM);
        return;
    };
    draw_ui_text_ex(
        "CURRENT ENVIRONMENT",
        INSPECTOR.x,
        202.0,
        TextStyle::new(8.0, dark::TEXT_DIM).params(),
    );
    let stats = [
        ("TEMPERATURE", format!("{:.1} C", planet.temperature)),
        ("ATMOSPHERE", format!("{:.2} ATM", planet.atmosphere)),
        ("WATER", format!("{:.0}%", planet.water * 100.0)),
        ("GRAVITY", format!("{:.2} G", planet.gravity)),
        ("RADIATION", format!("{:.2}", planet.radiation)),
        ("BIOSPHERE", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let y = 232.0 + index as f32 * 38.0;
        draw_ui_text_ex(
            label,
            INSPECTOR.x,
            y,
            TextStyle::new(9.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            value,
            INSPECTOR.right(),
            y,
            TextStyle::new(12.0, dark::TEXT_BRIGHT),
        );
        draw_line(
            INSPECTOR.x,
            y + 10.0,
            INSPECTOR.right(),
            y + 10.0,
            1.0,
            Color::new(0.18, 0.38, 0.43, 0.28),
        );
    }
    let selected = ctx.expanded_tool.and_then(|id| {
        ctx.data
            .terraforming_tools
            .iter()
            .find(|tool| tool.id == id && !tool_is_locked(tool, &ctx.session.completed_research))
    });
    if let Some(tool) = selected {
        draw_ui_text_ex(
            "NEXT INTERVENTION",
            INSPECTOR.x,
            475.0,
            TextStyle::new(8.0, dark::TEXT_DIM).params(),
        );
        draw_ui_text_ex(
            &tool.name,
            INSPECTOR.x,
            501.0,
            TextStyle::new(16.0, dark::TEXT_BRIGHT).params(),
        );
        draw_text_block(
            &effect_forecast(
                &tool.effect,
                ctx.tool_intensity,
                analysis_level(&ctx.session.completed_research),
            ),
            INSPECTOR.x,
            514.0,
            INSPECTOR.w,
            40.0,
            10.0,
            2.0,
            Color::new(0.50, 0.86, 0.96, 1.0),
        );
        draw_ui_text_ex(
            &format!(
                "{} CR  /  {} confidence",
                tool_cost(tool, ctx.tool_intensity),
                match analysis_level(&ctx.session.completed_research) {
                    0 => "LOW",
                    1 => "IMPROVING",
                    2 => "HIGH",
                    _ => "EXACT",
                }
            ),
            INSPECTOR.x,
            568.0,
            TextStyle::new(10.0, Color::new(1.0, 0.75, 0.38, 1.0)).params(),
        );
        if button(
            Rect::new(INSPECTOR.x, 590.0, INSPECTOR.w, 48.0),
            "APPLY TOOL",
            ctx.session.credits >= tool_cost(tool, ctx.tool_intensity),
            ButtonTone::Positive,
            mouse,
        ) {
            actions.push(UiAction::ApplyTool(tool.id.clone()));
        }
    } else {
        draw_text_block(
            "Choose a tool to preview its effect, confidence, and cost.",
            INSPECTOR.x,
            480.0,
            INSPECTOR.w,
            56.0,
            11.0,
            3.0,
            dark::TEXT_DIM,
        );
    }
    if button(
        Rect::new(INSPECTOR.x, 652.0, 176.0, 34.0),
        "ALIEN MARKET",
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Market));
    }
    if button(
        Rect::new(INSPECTOR.right() - 108.0, 652.0, 108.0, 34.0),
        "SALVAGE",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::ScrapPlanet);
    }
}
