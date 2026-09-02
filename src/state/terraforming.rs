use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolIntensity {
    Low,
    Standard,
    Heavy,
}

impl ToolIntensity {
    pub fn multiplier(self) -> f32 {
        match self {
            Self::Low => 0.5,
            Self::Standard => 1.0,
            Self::Heavy => 1.5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "LOW",
            Self::Standard => "STANDARD",
            Self::Heavy => "HEAVY",
        }
    }
}

impl GameSession {
    #[allow(dead_code)]
    pub fn apply_tool(&mut self, tool: &Tool) -> Result<String, String> {
        self.apply_tool_with_intensity(tool, ToolIntensity::Standard)
    }

    pub fn apply_tool_with_intensity(
        &mut self,
        tool: &Tool,
        intensity: ToolIntensity,
    ) -> Result<String, String> {
        self.require_started()?;
        if self.current_planet_id.is_none() {
            return Err("Select a planet first!".to_owned());
        }
        if tool_is_locked(tool, &self.completed_research) {
            return Err(format!("{} is locked.", tool.name));
        }
        let cost = tool_cost(tool, intensity);
        if self.credits < cost {
            return Err("Not enough credits!".to_owned());
        }
        let planet_id = self.current_planet_id.clone().expect("checked above");
        let planet = self
            .planets
            .iter_mut()
            .find(|planet| planet.id == planet_id)
            .ok_or_else(|| "That planet is no longer in your inventory.".to_owned())?;
        for (stat, delta) in tool.effect.iter().chain(tool.side_effects.iter()) {
            apply_stat(planet, stat, *delta * intensity.multiplier());
        }
        planet.invested_cost += cost;
        self.credits -= cost;
        self.stats.total_spend += cost;
        Ok(format!(
            "Used {} at {} intensity",
            tool.name,
            intensity.label()
        ))
    }
}

pub fn tool_cost(tool: &Tool, intensity: ToolIntensity) -> i64 {
    ((tool.cost.max(0) as f32) * intensity.multiplier()).round() as i64
}

pub fn forecast_range(value: f32, analysis: u32) -> (f32, f32) {
    let uncertainty = match analysis {
        0 => 0.25,
        1 => 0.12,
        2 => 0.05,
        _ => 0.0,
    };
    let spread = value.abs() * uncertainty;
    (value - spread, value + spread)
}

fn apply_stat(planet: &mut Planet, stat: &str, delta: f32) {
    match stat {
        "temperature" => planet.temperature = (planet.temperature + delta).clamp(-100.0, 200.0),
        "atmosphere" => planet.atmosphere = (planet.atmosphere + delta).clamp(0.0, 3.0),
        "water" => planet.water = (planet.water + delta).clamp(0.0, 1.0),
        "gravity" => planet.gravity = (planet.gravity + delta).clamp(0.1, 5.0),
        "radiation" => planet.radiation = (planet.radiation + delta).clamp(0.0, 2.0),
        "biosphere" => planet.biosphere = (planet.biosphere + delta).clamp(0.0, 3.0),
        _ => {}
    }
}
