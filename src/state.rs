//! Planet Trader's rules, mutable session, and save migration.

use crate::data::{GameConfig, GameData, PlanetType, ResearchDef, Species, SpeciesTemplate, Tool};
use macroquad::rand::gen_range;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Planet {
    pub id: String,
    pub planet_type: PlanetType,
    pub name: String,
    pub temperature: f32,
    pub atmosphere: f32,
    pub water: f32,
    pub gravity: f32,
    pub radiation: f32,
    #[serde(default)]
    pub biosphere: f32,
    pub purchase_price: i64,
    #[serde(default)]
    pub invested_cost: i64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlienBuyer {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub temp_range: [f32; 2],
    pub atmo_range: [f32; 2],
    pub water_range: [f32; 2],
    pub grav_range: [f32; 2],
    pub rad_range: [f32; 2],
    #[serde(default = "default_bio_range")]
    pub bio_range: [f32; 2],
    pub base_price: i64,
    pub current_price: i64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeRecord {
    pub transaction_type: String,
    pub planet_id: String,
    pub planet_name: String,
    pub buyer_name: Option<String>,
    pub purchase_cost: i64,
    pub invested_cost: i64,
    pub sale_price: i64,
    pub profit: i64,
    pub compatibility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradeStats {
    pub planets_purchased: u32,
    pub planets_sold: u32,
    pub planets_salvaged: u32,
    pub total_spend: i64,
    pub total_revenue: i64,
    pub total_profit: i64,
    pub best_profit: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TutorialStep {
    Welcome,
    BuyPlanet,
    ChooseOffer,
    SelectPlanet,
    InspectBuyer,
    UseTool,
    SellOrSalvage,
    OpenResearch,
    Complete,
}

impl TutorialStep {
    pub fn is_complete(self) -> bool {
        self == Self::Complete
    }
}

fn completed_tutorial() -> TutorialStep {
    TutorialStep::Complete
}

fn default_bio_range() -> [f32; 2] {
    [0.0, 3.0]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub game_started: bool,
    pub credits: i64,
    pub planets: Vec<Planet>,
    pub current_planet_id: Option<String>,
    #[serde(default)]
    pub stats: TradeStats,
    #[serde(default)]
    pub trade_history: Vec<TradeRecord>,
    #[serde(default)]
    pub alien_buyers: Vec<AlienBuyer>,
    #[serde(default)]
    pub research_points: i64,
    #[serde(default)]
    pub completed_research: Vec<String>,
    #[serde(default)]
    pub reputation: i64,
    #[serde(default = "completed_tutorial")]
    pub tutorial_step: TutorialStep,
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub game_started: bool,
    pub credits: i64,
    pub planets: Vec<Planet>,
    pub current_planet_id: Option<String>,
    pub planet_options: Vec<Planet>,
    pub planet_modal_open: bool,
    pub alien_buyers: Vec<AlienBuyer>,
    pub stats: TradeStats,
    pub trade_history: Vec<TradeRecord>,
    pub research_points: i64,
    pub completed_research: Vec<String>,
    pub reputation: i64,
    pub tutorial_step: TutorialStep,
    next_planet_id: u64,
    next_buyer_id: u64,
}

impl GameSession {
    pub fn new(data: &GameData) -> Self {
        let mut session = Self {
            game_started: false,
            credits: data.config.starting_credits,
            planets: Vec::new(),
            current_planet_id: None,
            planet_options: Vec::new(),
            planet_modal_open: false,
            alien_buyers: Vec::new(),
            stats: TradeStats::default(),
            trade_history: Vec::new(),
            research_points: 0,
            completed_research: Vec::new(),
            reputation: 0,
            tutorial_step: TutorialStep::Welcome,
            next_planet_id: 1,
            next_buyer_id: 1,
        };
        session.refresh_buyers(data);
        session
    }

    pub fn from_save(save: SaveData, data: &GameData) -> Self {
        let mut session = Self::new(data);
        session.game_started = save.game_started;
        session.credits = save.credits.max(0);
        session.planets = save.planets.into_iter().map(normalize_planet).collect();
        session.current_planet_id = save
            .current_planet_id
            .filter(|id| session.planets.iter().any(|planet| planet.id == *id));
        session.stats = save.stats;
        if session.stats.planets_purchased == 0 && !session.planets.is_empty() {
            session.stats.planets_purchased = session.planets.len() as u32;
            session.stats.total_spend = session
                .planets
                .iter()
                .map(|planet| planet.purchase_price)
                .sum();
        }
        session.trade_history = save.trade_history;
        session.research_points = save.research_points.max(0);
        let mut completed_research = Vec::new();
        for name in save.completed_research {
            let known = data.research.iter().any(|research| research.name == name);
            if known && !completed_research.iter().any(|entry| entry == &name) {
                completed_research.push(name);
            }
        }
        session.completed_research = completed_research;
        session.reputation = save.reputation.max(0);
        session.tutorial_step = save.tutorial_step;
        if !save.alien_buyers.is_empty() {
            session.alien_buyers = save.alien_buyers;
            for buyer in &mut session.alien_buyers {
                if buyer.bio_range == default_bio_range() {
                    if let Some(species) = data
                        .alien_species
                        .iter()
                        .find(|species| species.name == buyer.name)
                    {
                        buyer.bio_range = species.bio_range;
                    }
                }
            }
        }
        session.next_planet_id = next_id(&session.planets);
        session.next_buyer_id = session
            .alien_buyers
            .iter()
            .map(|buyer| buyer.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        session.planet_options.clear();
        session.planet_modal_open = false;
        session
    }

    pub fn to_save(&self, version: &str) -> SaveData {
        SaveData {
            version: version.to_owned(),
            game_started: self.game_started,
            credits: self.credits,
            planets: self.planets.clone(),
            current_planet_id: self.current_planet_id.clone(),
            stats: self.stats.clone(),
            trade_history: self.trade_history.clone(),
            alien_buyers: self.alien_buyers.clone(),
            research_points: self.research_points,
            completed_research: self.completed_research.clone(),
            reputation: self.reputation,
            tutorial_step: self.tutorial_step,
        }
    }

    pub fn current_planet(&self) -> Option<&Planet> {
        self.current_planet_id
            .as_ref()
            .and_then(|id| self.planets.iter().find(|planet| &planet.id == id))
    }

    pub fn refresh_buyers(&mut self, data: &GameData) {
        if !data.alien_species.is_empty() {
            let count = data.alien_species.len().min(4);
            let chosen = unique_indices(data.alien_species.len(), count);
            self.alien_buyers = chosen
                .into_iter()
                .map(|index| self.create_buyer_from_species(&data.alien_species[index]))
                .collect();
            return;
        }

        let count = data.alien_species_types.len().min(4);
        if count == 0 {
            self.alien_buyers.clear();
        } else {
            let chosen = unique_indices(data.alien_species_types.len(), count);
            self.alien_buyers = chosen
                .into_iter()
                .map(|index| self.create_buyer_from_template(&data.alien_species_types[index]))
                .collect();
        }
    }

    pub fn open_purchase_modal(&mut self, data: &GameData) -> Result<(), String> {
        if !self.game_started {
            return Err("Complete the tutorial first!".to_owned());
        }
        if data.planet_types.is_empty() || data.planet_names.is_empty() {
            return Err("Planet catalogue is still loading.".to_owned());
        }

        let count = contract_option_count(self.reputation);
        self.planet_options = (0..count)
            .map(|_| {
                let type_index = gen_range(0, data.planet_types.len() as i32) as usize;
                let name_index = gen_range(0, data.planet_names.len() as i32) as usize;
                let id = format!("planet-{}", self.next_planet_id);
                self.next_planet_id += 1;
                Planet::from_type(
                    id,
                    data.planet_types[type_index].clone(),
                    data.planet_names[name_index].clone(),
                )
            })
            .collect();
        self.planet_modal_open = true;
        Ok(())
    }

    pub fn close_purchase_modal(&mut self) {
        self.planet_modal_open = false;
    }

    pub fn purchase_planet(&mut self, planet_id: &str) -> Result<String, String> {
        self.require_started()?;
        let index = self
            .planet_options
            .iter()
            .position(|planet| planet.id == planet_id)
            .ok_or_else(|| "That planet is no longer available.".to_owned())?;
        let planet = self.planet_options.remove(index);
        if self.credits < planet.purchase_price {
            self.planet_options.insert(index, planet);
            return Err("Not enough credits for that planet.".to_owned());
        }

        self.credits -= planet.purchase_price;
        let name = planet.name.clone();
        let price = planet.purchase_price;
        self.stats.planets_purchased += 1;
        self.stats.total_spend += price;
        self.record_trade(TradeRecord {
            transaction_type: "purchase".to_owned(),
            planet_id: planet.id.clone(),
            planet_name: planet.name.clone(),
            buyer_name: None,
            purchase_cost: price,
            invested_cost: price,
            sale_price: 0,
            profit: -price,
            compatibility: 0.0,
        });
        self.planets.push(planet);
        self.planet_modal_open = false;
        Ok(format!("Purchased {} for {} CR", name, price))
    }

    pub fn select_planet(&mut self, planet_id: &str) -> Result<String, String> {
        self.require_started()?;
        let planet = self
            .planets
            .iter()
            .find(|planet| planet.id == planet_id)
            .ok_or_else(|| "That planet is not in your inventory.".to_owned())?;
        self.current_planet_id = Some(planet.id.clone());
        Ok(format!("Selected {}", planet.name))
    }

    pub fn apply_tool(&mut self, tool: &Tool) -> Result<String, String> {
        self.require_started()?;
        if self.current_planet_id.is_none() {
            return Err("Select a planet first!".to_owned());
        }
        if tool_is_locked(tool, &self.completed_research) {
            return Err(format!("{} is locked.", tool.name));
        }
        if self.credits < tool.cost {
            return Err("Not enough credits!".to_owned());
        }

        let planet_id = self.current_planet_id.clone().expect("checked above");
        let planet = self
            .planets
            .iter_mut()
            .find(|planet| planet.id == planet_id)
            .ok_or_else(|| "That planet is no longer in your inventory.".to_owned())?;
        for (stat, delta) in tool.effect.iter().chain(tool.side_effects.iter()) {
            apply_stat(planet, stat, *delta);
        }
        planet.invested_cost += tool.cost;
        self.credits -= tool.cost;
        self.stats.total_spend += tool.cost;
        Ok(format!("Used {}", tool.name))
    }

    pub fn complete_research(&mut self, research: &ResearchDef) -> Result<String, String> {
        self.require_started()?;
        if self.research_is_complete(&research.name) {
            return Err(format!("{} is already researched.", research.name));
        }

        let cost = research.rp_cost.max(0);
        if self.research_points < cost {
            return Err(format!("{} research points needed.", cost));
        }

        self.research_points -= cost;
        self.completed_research.push(research.name.clone());
        Ok(format!("Research complete: {}", research.name))
    }

    pub fn research_is_complete(&self, name: &str) -> bool {
        self.completed_research.iter().any(|entry| entry == name)
    }

    pub fn sell_planet(&mut self, buyer_id: u64) -> Result<String, String> {
        self.require_started()?;
        let planet_id = self
            .current_planet_id
            .clone()
            .ok_or_else(|| "No planet selected to sell.".to_owned())?;
        let buyer = self
            .alien_buyers
            .iter()
            .find(|buyer| buyer.id == buyer_id)
            .cloned()
            .ok_or_else(|| "That buyer has left the market.".to_owned())?;
        let (name, purchase_cost, invested_cost, compatibility_score) = {
            let planet = self
                .planets
                .iter()
                .find(|planet| planet.id == planet_id)
                .ok_or_else(|| "That planet is no longer in your inventory.".to_owned())?;
            (
                planet.name.clone(),
                planet.purchase_price,
                investment_cost(planet),
                compatibility(planet, &buyer),
            )
        };
        if compatibility_score < 0.6 {
            return Err("This buyer's requirements are not met.".to_owned());
        }

        let price = sale_price_for_score(&buyer, compatibility_score);
        let profit = price - invested_cost;
        let research_award = research_points_for_sale(price, compatibility_score);
        self.credits += price;
        self.research_points = self.research_points.saturating_add(research_award);
        let reputation_award = reputation_for_sale(profit, compatibility_score);
        self.reputation = self.reputation.saturating_add(reputation_award);
        self.stats.planets_sold += 1;
        self.stats.total_revenue += price;
        self.stats.total_profit += profit;
        self.stats.best_profit = self.stats.best_profit.max(profit);
        self.record_trade(TradeRecord {
            transaction_type: "sale".to_owned(),
            planet_id: planet_id.clone(),
            planet_name: name.clone(),
            buyer_name: Some(buyer.name.clone()),
            purchase_cost,
            invested_cost,
            sale_price: price,
            profit,
            compatibility: compatibility_score,
        });
        self.planets.retain(|planet| planet.id != planet_id);
        self.current_planet_id = None;
        Ok(format!(
            "Sold {} to {} for {} CR (+{} RP, +{} REP)",
            name, buyer.name, price, research_award, reputation_award
        ))
    }

    pub fn salvage_current_planet(&mut self) -> Result<String, String> {
        self.require_started()?;
        let planet_id = self
            .current_planet_id
            .clone()
            .ok_or_else(|| "No planet selected to salvage.".to_owned())?;
        let (name, purchase_cost, invested_cost) = {
            let planet = self
                .planets
                .iter()
                .find(|planet| planet.id == planet_id)
                .ok_or_else(|| "That planet is no longer in your inventory.".to_owned())?;
            (
                planet.name.clone(),
                planet.purchase_price,
                investment_cost(planet),
            )
        };
        let salvage_price = ((invested_cost as f32) * 0.25).round().max(100.0) as i64;
        let profit = salvage_price - invested_cost;
        self.credits += salvage_price;
        self.stats.planets_salvaged += 1;
        self.stats.total_revenue += salvage_price;
        self.stats.total_profit += profit;
        self.record_trade(TradeRecord {
            transaction_type: "salvage".to_owned(),
            planet_id: planet_id.clone(),
            planet_name: name.clone(),
            buyer_name: Some("Salvage Broker".to_owned()),
            purchase_cost,
            invested_cost,
            sale_price: salvage_price,
            profit,
            compatibility: 0.0,
        });
        self.planets.retain(|planet| planet.id != planet_id);
        self.current_planet_id = None;
        Ok(format!("Salvaged {} for {} CR", name, salvage_price))
    }

    fn record_trade(&mut self, record: TradeRecord) {
        const MAX_HISTORY: usize = 50;
        self.trade_history.push(record);
        if self.trade_history.len() > MAX_HISTORY {
            let excess = self.trade_history.len() - MAX_HISTORY;
            self.trade_history.drain(0..excess);
        }
    }

    fn require_started(&self) -> Result<(), String> {
        if self.game_started {
            Ok(())
        } else {
            Err("Complete the tutorial first!".to_owned())
        }
    }

    fn create_buyer_from_species(&mut self, species: &Species) -> AlienBuyer {
        let base_price = species.base_price.max(100);
        let spread = ((base_price as f32) * 0.25) as i64;
        let current_price = (base_price + gen_range(-spread, spread + 1)).max(100);
        let buyer = AlienBuyer {
            id: self.next_buyer_id,
            name: species.name.clone(),
            description: species.description.clone(),
            temp_range: species.temp_range,
            atmo_range: species.atmo_range,
            water_range: species.water_range,
            grav_range: species.grav_range,
            rad_range: species.rad_range,
            bio_range: species.bio_range,
            base_price,
            current_price,
            color: species.color.clone(),
        };
        self.next_buyer_id += 1;
        buyer
    }

    fn create_buyer_from_template(&mut self, template: &SpeciesTemplate) -> AlienBuyer {
        let prefix = random_string(&template.prefixes, "Market");
        let suffix = random_string(&template.suffixes, "Buyer");
        let base_price = 5000;
        let spread = ((base_price as f32) * 0.25) as i64;
        let current_price = (base_price + gen_range(-spread, spread + 1)).max(100);
        let buyer = AlienBuyer {
            id: self.next_buyer_id,
            name: format!("{} {}", prefix, suffix),
            description: template.desc.clone(),
            temp_range: template.temp,
            atmo_range: template.atmo,
            water_range: template.water,
            grav_range: template.grav,
            rad_range: template.rad,
            bio_range: template.bio,
            base_price,
            current_price,
            color: random_string(&template.colors, "#40B8C8"),
        };
        self.next_buyer_id += 1;
        buyer
    }
}

impl Planet {
    fn from_type(id: String, planet_type: PlanetType, name: String) -> Self {
        let purchase_price = gen_range(1000, 3000) as i64;
        Self {
            id,
            name,
            temperature: planet_type.base_temp + gen_range(-10.0, 10.0),
            atmosphere: (planet_type.base_atmo + gen_range(-0.2, 0.2)).max(0.0),
            water: (planet_type.base_water + gen_range(-0.15, 0.15)).clamp(0.0, 1.0),
            gravity: (planet_type.base_grav + gen_range(-0.2, 0.2)).max(0.1),
            radiation: (planet_type.base_rad + gen_range(-0.15, 0.15)).max(0.0),
            biosphere: 0.0,
            purchase_price,
            invested_cost: purchase_price,
            color: planet_type.color.clone(),
            planet_type,
        }
    }
}

pub fn compatibility(planet: &Planet, buyer: &AlienBuyer) -> f32 {
    let matches = [
        within(planet.temperature, buyer.temp_range),
        within(planet.atmosphere, buyer.atmo_range),
        within(planet.water, buyer.water_range),
        within(planet.gravity, buyer.grav_range),
        within(planet.radiation, buyer.rad_range),
        within(planet.biosphere, buyer.bio_range),
    ];
    matches.into_iter().filter(|matched| *matched).count() as f32 / 6.0
}

pub fn sale_price(planet: &Planet, buyer: &AlienBuyer) -> i64 {
    sale_price_for_score(buyer, compatibility(planet, buyer))
}

pub fn potential_profit(planet: &Planet, buyer: &AlienBuyer) -> i64 {
    sale_price(planet, buyer) - investment_cost(planet)
}

pub fn market_trend_percent(buyer: &AlienBuyer) -> f32 {
    if buyer.base_price <= 0 {
        0.0
    } else {
        ((buyer.current_price - buyer.base_price) as f32 / buyer.base_price as f32) * 100.0
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

fn within(value: f32, range: [f32; 2]) -> bool {
    value >= range[0] && value <= range[1]
}

fn sale_price_for_score(buyer: &AlienBuyer, score: f32) -> i64 {
    let compatibility_multiplier = 0.5 + score.clamp(0.0, 1.0) * 1.5;
    ((buyer.current_price.max(100) as f32) * compatibility_multiplier)
        .round()
        .max(100.0) as i64
}

pub fn projected_research_points(price: i64, compatibility_score: f32) -> i64 {
    let market_value = (price.max(0) as f32 / 500.0).floor() as i64;
    let quality = (compatibility_score.clamp(0.0, 1.0) * 12.0).round() as i64;
    let perfect_bonus = if compatibility_score >= 0.999 { 8 } else { 0 };
    (market_value + quality + perfect_bonus).max(5)
}

fn research_points_for_sale(price: i64, compatibility_score: f32) -> i64 {
    projected_research_points(price, compatibility_score)
}

fn reputation_for_sale(profit: i64, compatibility_score: f32) -> i64 {
    let quality = (compatibility_score.clamp(0.0, 1.0) * 10.0).round() as i64;
    let profit_bonus = (profit.max(0) / 2_000).min(8);
    4 + quality + profit_bonus
}

fn investment_cost(planet: &Planet) -> i64 {
    planet.invested_cost.max(planet.purchase_price)
}

fn normalize_planet(mut planet: Planet) -> Planet {
    planet.invested_cost = investment_cost(&planet);
    planet
}

fn next_id(planets: &[Planet]) -> u64 {
    planets
        .iter()
        .filter_map(|planet| planet.id.strip_prefix("planet-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0)
        .saturating_add(1)
}

fn unique_indices(length: usize, count: usize) -> Vec<usize> {
    let mut chosen = Vec::with_capacity(count);
    while chosen.len() < count {
        let index = gen_range(0, length as i32) as usize;
        if !chosen.contains(&index) {
            chosen.push(index);
        }
    }
    chosen
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

fn random_string(values: &[String], fallback: &str) -> String {
    if values.is_empty() {
        fallback.to_owned()
    } else {
        values[gen_range(0, values.len() as i32) as usize].clone()
    }
}

#[derive(Debug, Deserialize)]
struct LegacySave {
    #[serde(default)]
    game_started: Option<bool>,
    #[serde(default)]
    credits: Option<i64>,
    #[serde(default)]
    planets: Option<Vec<Planet>>,
    #[serde(default)]
    current_planet_id: Option<String>,
}

pub fn migrate_save_value(
    detected_version: Option<String>,
    value: Value,
    config: &GameConfig,
) -> Result<SaveData, String> {
    let payload = value.get("data").cloned().unwrap_or(value);
    if let Ok(mut current) = serde_json::from_value::<SaveData>(payload.clone()) {
        current.version = config.version.clone();
        return Ok(current);
    }

    let legacy: LegacySave = serde_json::from_value(payload)
        .map_err(|error| format!("Unsupported save format {:?}: {}", detected_version, error))?;
    Ok(SaveData {
        version: config.version.clone(),
        game_started: legacy.game_started.unwrap_or(false),
        credits: legacy.credits.unwrap_or(config.starting_credits).max(0),
        planets: legacy.planets.unwrap_or_default(),
        current_planet_id: legacy.current_planet_id,
        stats: TradeStats::default(),
        trade_history: Vec::new(),
        alien_buyers: Vec::new(),
        research_points: 0,
        completed_research: Vec::new(),
        reputation: 0,
        tutorial_step: TutorialStep::Complete,
    })
}

#[cfg(test)]
mod tests;
