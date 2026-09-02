use crate::state::TutorialStep;
use crate::ui::AppScreen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CaptureSceneSeed {
    pub(super) reset_session: bool,
    pub(super) game_started: bool,
    pub(super) research_points: i64,
    pub(super) first_research_complete: bool,
    pub(super) research_open: bool,
    pub(super) planet_demo: bool,
    pub(super) reset_open: bool,
    pub(super) screen: AppScreen,
    pub(super) settings_open: bool,
    pub(super) tutorial_step: TutorialStep,
}

impl CaptureSceneSeed {
    pub(super) fn for_scene(scene: &str) -> Self {
        match scene {
            "research" => Self::new(
                true,
                115,
                true,
                false,
                AppScreen::Gameplay,
                TutorialStep::Complete,
            ),
            "home" | "title" => Self::new(
                false,
                0,
                false,
                false,
                AppScreen::Home,
                TutorialStep::Complete,
            ),
            "tutorial" => Self::new(
                true,
                0,
                false,
                false,
                AppScreen::Gameplay,
                TutorialStep::Welcome,
            ),
            "tutorial_buy" => Self::new(
                true,
                0,
                false,
                false,
                AppScreen::Gameplay,
                TutorialStep::BuyPlanet,
            ),
            "tutorial_market" => Self::new(
                true,
                0,
                false,
                true,
                AppScreen::Gameplay,
                TutorialStep::InspectBuyer,
            ),
            "settings" => {
                let mut seed = Self::new(
                    false,
                    0,
                    false,
                    false,
                    AppScreen::Home,
                    TutorialStep::Complete,
                );
                seed.settings_open = true;
                seed
            }
            "biosphere" | "market" | "advanced" => Self::new(
                true,
                0,
                false,
                true,
                AppScreen::Gameplay,
                TutorialStep::Complete,
            ),
            "reset" => {
                let mut seed = Self::new(
                    true,
                    0,
                    false,
                    false,
                    AppScreen::Gameplay,
                    TutorialStep::Complete,
                );
                seed.reset_open = true;
                seed
            }
            _ => Self::new(
                true,
                0,
                false,
                false,
                AppScreen::Gameplay,
                TutorialStep::Complete,
            ),
        }
    }

    fn new(
        game_started: bool,
        research_points: i64,
        first_research_complete: bool,
        planet_demo: bool,
        screen: AppScreen,
        tutorial_step: TutorialStep,
    ) -> Self {
        Self {
            reset_session: true,
            game_started,
            research_points,
            first_research_complete,
            research_open: false,
            planet_demo,
            reset_open: false,
            screen,
            settings_open: false,
            tutorial_step,
        }
    }
}
