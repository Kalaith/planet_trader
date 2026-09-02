use super::*;

const TOOL_RAIL: Rect = Rect::new(18.0, 150.0, 360.0, 540.0);
const PLANET_STAGE: Rect = Rect::new(394.0, 150.0, 540.0, 540.0);
const INSPECTOR: Rect = Rect::new(950.0, 150.0, 312.0, 540.0);

pub(super) fn draw_workshop(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_tool_rail(ctx, mouse, actions);
    draw_planet_stage(ctx, mouse, actions);
    draw_inspector(ctx, mouse, actions);
}

fn draw_tool_rail(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel_with_left_title(TOOL_RAIL, "TERRAFORMING ARRAY");
    let unlocked: Vec<_> = ctx
        .data
        .terraforming_tools
        .iter()
        .filter(|tool| !tool_is_locked(tool, &ctx.session.completed_research))
        .collect();
    draw_ui_text_ex(
        &format!("{} operational tools", unlocked.len()),
        TOOL_RAIL.x + 14.0,
        TOOL_RAIL.y + 60.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    if button(
        Rect::new(TOOL_RAIL.right() - 104.0, TOOL_RAIL.y + 9.0, 90.0, 30.0),
        "RESEARCH",
        true,
        ButtonTone::Primary,
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
            Rect::new(
                TOOL_RAIL.x + 12.0 + index as f32 * 113.0,
                TOOL_RAIL.y + 72.0,
                106.0,
                30.0,
            ),
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
            "LOW / HEAVY unlock with Systems Modeling",
            TOOL_RAIL.x + 14.0,
            TOOL_RAIL.y + 118.0,
            TextStyle::new(9.0, Color::new(0.76, 0.61, 0.32, 1.0)).params(),
        );
    }

    let list = Rect::new(
        TOOL_RAIL.x + 10.0,
        TOOL_RAIL.y + 130.0,
        TOOL_RAIL.w - 20.0,
        TOOL_RAIL.h - 142.0,
    );
    let total: f32 = unlocked
        .iter()
        .map(|tool| {
            if ctx.expanded_tool == Some(tool.id.as_str()) {
                152.0
            } else {
                64.0
            }
        })
        .sum::<f32>()
        + unlocked.len().saturating_sub(1) as f32 * 8.0;
    let max_scroll = (total - list.h).max(0.0);
    let scroll = ctx.tool_scroll.min(max_scroll);
    let mut y = list.y - scroll;
    for tool in unlocked {
        let selected = ctx.expanded_tool == Some(tool.id.as_str());
        let height = if selected { 152.0 } else { 64.0 };
        let card = Rect::new(list.x, y, list.w, height);
        if card.bottom() >= list.y && card.y <= list.bottom() {
            draw_tool_choice(ctx, tool, card, selected, mouse, actions);
        }
        y += height + 8.0;
    }
    if max_scroll > 0.0 {
        if button(
            Rect::new(TOOL_RAIL.right() - 76.0, TOOL_RAIL.y + 108.0, 30.0, 24.0),
            "UP",
            scroll > 0.0,
            ButtonTone::Muted,
            mouse,
        ) {
            actions.push(UiAction::ScrollTools(-1));
        }
        if button(
            Rect::new(TOOL_RAIL.right() - 40.0, TOOL_RAIL.y + 108.0, 30.0, 24.0),
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
    card: Rect,
    selected: bool,
    mouse: Vec2,
    actions: &mut Vec<UiAction>,
) {
    let hovered = card.contains_point(mouse);
    draw_surface(
        card,
        &SurfaceStyle::new(if selected {
            Color::new(0.08, 0.24, 0.32, 1.0)
        } else if hovered {
            Color::new(0.10, 0.18, 0.24, 1.0)
        } else {
            Color::new(0.065, 0.12, 0.16, 1.0)
        })
        .with_border(
            1.0,
            if selected {
                Color::new(0.32, 0.82, 1.0, 1.0)
            } else {
                Color::new(0.18, 0.36, 0.44, 1.0)
            },
        )
        .with_left_accent(4.0, Color::new(0.20, 0.72, 0.90, 1.0)),
    );
    draw_ui_text_ex(
        &format!("{}  {}", tool_icon(&tool.category), tool.name),
        card.x + 12.0,
        card.y + 22.0,
        TextStyle::new(14.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{} CR  //  {}",
            tool_cost(tool, ctx.tool_intensity),
            ctx.tool_intensity.label()
        ),
        card.x + 12.0,
        card.y + 43.0,
        TextStyle::new(10.0, Color::new(0.66, 0.78, 0.82, 1.0)).params(),
    );
    if selected {
        draw_text_block(
            &tool.description,
            card.x + 12.0,
            card.y + 60.0,
            card.w - 24.0,
            38.0,
            10.0,
            2.0,
            dark::TEXT_DIM,
        );
        draw_text_block(
            &format!(
                "PRIMARY  {}\nSIDE EFFECTS  {}",
                effect_forecast(
                    &tool.effect,
                    ctx.tool_intensity,
                    analysis_level(&ctx.session.completed_research)
                ),
                effect_forecast(
                    &tool.side_effects,
                    ctx.tool_intensity,
                    analysis_level(&ctx.session.completed_research)
                )
            ),
            card.x + 12.0,
            card.y + 106.0,
            card.w - 24.0,
            38.0,
            9.0,
            2.0,
            Color::new(0.56, 0.78, 0.84, 1.0),
        );
    }
    if hovered && is_mouse_button_released(MouseButton::Left) {
        actions.push(UiAction::ToggleTool(tool.id.clone()));
    }
}

fn draw_planet_stage(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    draw_panel(PLANET_STAGE, "ACTIVE WORLD");
    let Some(planet) = ctx.session.current_planet() else {
        draw_text_centered_in_box(
            "NO WORLD IN THE CRADLE",
            PLANET_STAGE.x + 50.0,
            PLANET_STAGE.y + 150.0,
            PLANET_STAGE.w - 100.0,
            42.0,
            22.0,
            dark::TEXT_DIM,
        );
        draw_text_centered_in_box(
            "Acquire a frontier contract, then select the world from your portfolio.",
            PLANET_STAGE.x + 90.0,
            PLANET_STAGE.y + 205.0,
            PLANET_STAGE.w - 180.0,
            64.0,
            13.0,
            dark::TEXT_DIM,
        );
        if button(
            Rect::new(
                PLANET_STAGE.center().x - 100.0,
                PLANET_STAGE.y + 300.0,
                200.0,
                48.0,
            ),
            "SCAN CONTRACTS",
            true,
            ButtonTone::Primary,
            mouse,
        ) {
            actions.push(UiAction::OpenPurchase);
        }
        draw_portfolio(ctx, mouse, actions);
        return;
    };
    draw_ui_text_ex(
        &planet.name,
        PLANET_STAGE.x + 18.0,
        PLANET_STAGE.y + 72.0,
        TextStyle::new(20.0, dark::TEXT_BRIGHT).params(),
    );
    draw_ui_text_ex(
        &format!(
            "{}  //  {} CR invested",
            planet.planet_type.name, planet.invested_cost
        ),
        PLANET_STAGE.x + 20.0,
        PLANET_STAGE.y + 94.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    draw_planet_orb(
        planet,
        vec2(PLANET_STAGE.center().x, PLANET_STAGE.y + 235.0),
        138.0,
    );
    if let Some((buyer, score, estimate, profit)) =
        best_market_route(planet, &ctx.session.alien_buyers)
    {
        let matches = compatibility_matches(score);
        draw_badge(
            Rect::new(
                PLANET_STAGE.x + 58.0,
                PLANET_STAGE.y + 386.0,
                PLANET_STAGE.w - 116.0,
                38.0,
            ),
            &format!(
                "BEST ROUTE  {}  //  {}/6  //  {:+} CR",
                buyer.name, matches, profit
            ),
            if matches >= 4 {
                Color::new(0.06, 0.24, 0.16, 1.0)
            } else {
                Color::new(0.24, 0.16, 0.06, 1.0)
            },
            if matches >= 4 {
                Color::new(0.48, 1.0, 0.64, 1.0)
            } else {
                Color::new(1.0, 0.76, 0.38, 1.0)
            },
        );
        let _ = estimate;
    }
    draw_portfolio(ctx, mouse, actions);
}

fn draw_portfolio(ctx: &UiContext<'_>, mouse: Vec2, actions: &mut Vec<UiAction>) {
    let strip = Rect::new(
        PLANET_STAGE.x + 12.0,
        PLANET_STAGE.bottom() - 102.0,
        PLANET_STAGE.w - 24.0,
        88.0,
    );
    draw_surface(
        strip,
        &SurfaceStyle::new(Color::new(0.04, 0.085, 0.12, 1.0))
            .with_border(1.0, Color::new(0.16, 0.34, 0.42, 1.0)),
    );
    draw_ui_text_ex(
        "PORTFOLIO",
        strip.x + 10.0,
        strip.y + 20.0,
        TextStyle::new(10.0, dark::TEXT_DIM).params(),
    );
    if ctx.session.planets.is_empty() {
        draw_ui_text_ex(
            "No owned worlds",
            strip.x + 10.0,
            strip.y + 52.0,
            TextStyle::new(13.0, dark::TEXT_DIM).params(),
        );
        return;
    }
    let start = ctx
        .inventory_scroll
        .min(ctx.session.planets.len().saturating_sub(1));
    for (index, planet) in ctx.session.planets.iter().skip(start).take(3).enumerate() {
        let card = Rect::new(
            strip.x + 8.0 + index as f32 * 166.0,
            strip.y + 30.0,
            158.0,
            46.0,
        );
        let selected = ctx.session.current_planet_id.as_deref() == Some(planet.id.as_str());
        if button(
            card,
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
    draw_panel(INSPECTOR, "OUTCOME PREVIEW");
    let Some(planet) = ctx.session.current_planet() else {
        draw_text_block("Select an owned world to inspect known environmental systems and estimate an intervention.", INSPECTOR.x + 16.0, INSPECTOR.y + 72.0, INSPECTOR.w - 32.0, 90.0, 13.0, 4.0, dark::TEXT_DIM);
        return;
    };
    let stats = [
        ("TEMPERATURE", format!("{:.1} C", planet.temperature)),
        ("ATMOSPHERE", format!("{:.2} ATM", planet.atmosphere)),
        ("WATER", format!("{:.0}%", planet.water * 100.0)),
        ("GRAVITY", format!("{:.2} G", planet.gravity)),
        ("RADIATION", format!("{:.2}", planet.radiation)),
        ("BIOSPHERE", format!("{:.1}", planet.biosphere)),
    ];
    for (index, (label, value)) in stats.iter().enumerate() {
        let row = Rect::new(
            INSPECTOR.x + 14.0,
            INSPECTOR.y + 58.0 + index as f32 * 39.0,
            INSPECTOR.w - 28.0,
            32.0,
        );
        draw_surface(
            row,
            &SurfaceStyle::new(Color::new(0.06, 0.12, 0.16, 1.0))
                .with_border(1.0, Color::new(0.16, 0.31, 0.38, 1.0)),
        );
        draw_ui_text_ex(
            label,
            row.x + 9.0,
            row.y + 21.0,
            TextStyle::new(9.0, dark::TEXT_DIM).params(),
        );
        draw_text_right(
            value,
            row.right() - 9.0,
            row.y + 21.0,
            TextStyle::new(13.0, dark::TEXT_BRIGHT),
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
            "ESTIMATED CHANGE",
            INSPECTOR.x + 16.0,
            INSPECTOR.y + 310.0,
            TextStyle::new(10.0, dark::TEXT_DIM).params(),
        );
        draw_text_block(
            &effect_forecast(
                &tool.effect,
                ctx.tool_intensity,
                analysis_level(&ctx.session.completed_research),
            ),
            INSPECTOR.x + 16.0,
            INSPECTOR.y + 326.0,
            INSPECTOR.w - 32.0,
            52.0,
            11.0,
            3.0,
            Color::new(0.50, 0.86, 0.96, 1.0),
        );
        draw_ui_text_ex(
            &format!(
                "Cost {} CR  //  model confidence {}",
                tool_cost(tool, ctx.tool_intensity),
                match analysis_level(&ctx.session.completed_research) {
                    0 => "LOW",
                    1 => "IMPROVING",
                    2 => "HIGH",
                    _ => "EXACT",
                }
            ),
            INSPECTOR.x + 16.0,
            INSPECTOR.y + 392.0,
            TextStyle::new(10.0, Color::new(1.0, 0.75, 0.38, 1.0)).params(),
        );
        let usable = ctx.session.credits >= tool_cost(tool, ctx.tool_intensity);
        if button(
            Rect::new(
                INSPECTOR.x + 14.0,
                INSPECTOR.bottom() - 112.0,
                INSPECTOR.w - 28.0,
                48.0,
            ),
            "APPLY TOOL",
            usable,
            ButtonTone::Positive,
            mouse,
        ) {
            actions.push(UiAction::ApplyTool(tool.id.clone()));
        }
    } else {
        draw_text_block("Select a tool on the left to preview its primary effect, side effects, confidence, and cost.", INSPECTOR.x + 16.0, INSPECTOR.y + 318.0, INSPECTOR.w - 32.0, 74.0, 12.0, 4.0, dark::TEXT_DIM);
    }
    if button(
        Rect::new(INSPECTOR.x + 14.0, INSPECTOR.bottom() - 52.0, 172.0, 36.0),
        "ALIEN MARKET",
        true,
        ButtonTone::Primary,
        mouse,
    ) {
        actions.push(UiAction::SetMode(GameplayMode::Market));
    }
    if button(
        Rect::new(
            INSPECTOR.right() - 110.0,
            INSPECTOR.bottom() - 52.0,
            96.0,
            36.0,
        ),
        "SALVAGE",
        true,
        ButtonTone::Secondary,
        mouse,
    ) {
        actions.push(UiAction::ScrapPlanet);
    }
}
