use super::*;

pub const KNOWLEDGE_FIELDS: [(&str, &str); 6] = [
    ("frontier", "Frontier Science"),
    ("hydrology", "Hydrology"),
    ("volcanology", "Volcanology"),
    ("atmospherics", "Atmospherics"),
    ("harsh-world", "Harsh Worlds"),
    ("ecology", "Ecology"),
];

impl GameSession {
    pub fn research_is_complete(&self, name: &str) -> bool {
        self.completed_research.iter().any(|entry| entry == name)
    }

    pub fn knowledge(&self, field: &str) -> u32 {
        self.species_knowledge.get(field).copied().unwrap_or(0)
    }

    pub fn research_is_discovered(&self, research: &ResearchDef) -> bool {
        research.knowledge_required == 0
            || self.knowledge(&research.branch) >= research.knowledge_required
            || self.research_is_complete(&research.name)
    }

    pub fn research_prerequisite_met(&self, research: &ResearchDef) -> bool {
        research
            .prerequisite
            .as_deref()
            .is_none_or(|name| self.research_is_complete(name))
    }

    pub fn complete_research(&mut self, research: &ResearchDef) -> Result<String, String> {
        self.require_started()?;
        if self.research_is_complete(&research.name) {
            return Err(format!("{} is already researched.", research.name));
        }
        if !self.research_is_discovered(research) {
            return Err("This field has not been discovered yet.".to_owned());
        }
        if !self.research_prerequisite_met(research) {
            return Err(format!(
                "Research {} first.",
                research
                    .prerequisite
                    .as_deref()
                    .unwrap_or("the prerequisite")
            ));
        }
        let rp = research.rp_cost.max(0);
        let credits = research.credit_cost.max(0);
        if self.research_points < rp {
            return Err(format!("{} research points needed.", rp));
        }
        if self.credits < credits {
            return Err(format!("{} credits needed.", credits));
        }
        self.research_points -= rp;
        self.credits -= credits;
        self.completed_research.push(research.name.clone());
        Ok(format!("Research complete: {}", research.name))
    }
}

pub fn company_rank(reputation: i64) -> (&'static str, i64) {
    match reputation.max(0) {
        0..=24 => ("Frontier Startup", 25),
        25..=59 => ("Established Broker", 60),
        60..=119 => ("Renowned Terraformer", 120),
        _ => ("Stellar Institution", 120),
    }
}

pub fn contract_option_count(reputation: i64) -> usize {
    match reputation.max(0) {
        0..=24 => 3,
        25..=59 => 4,
        _ => 5,
    }
}

pub fn compatibility_matches(score: f32) -> u32 {
    (score.clamp(0.0, 1.0) * 6.0).round() as u32
}

pub fn knowledge_award_for_matches(matches: u32) -> u32 {
    match matches {
        6.. => 3,
        5 => 2,
        4 => 1,
        _ => 0,
    }
}

pub fn analysis_level(completed: &[String]) -> u32 {
    if completed
        .iter()
        .any(|name| name == "Precision Climate Grid")
    {
        3
    } else if completed
        .iter()
        .any(|name| name == "Planetary Systems Modeling")
    {
        2
    } else if completed.iter().any(|name| name == "Atmospheric Chemistry") {
        1
    } else {
        0
    }
}

pub fn tool_is_locked(tool: &Tool, completed_research: &[String]) -> bool {
    if tool.unlocked {
        return false;
    }
    let unlocked_by = [Some(tool.name.as_str()), tool.upgrade_required.as_deref()];
    !unlocked_by
        .into_iter()
        .flatten()
        .any(|name| completed_research.iter().any(|entry| entry == name))
}
