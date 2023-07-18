const MIN_SKILL: f32 = 0.25;
const MAX_SKILL: f32 = 10.0;

struct Skill {
    talent: f32,
    accumulated_xp: f32,
    skill: f32,
}

impl Skill {
    pub fn new(talent: f32, initial_xp: f32) -> Self {
        debug_assert!(talent > 0.0);

        let mut this = Self {
            talent,
            accumulated_xp: initial_xp,
            skill: 0.0,
        };

        this.update_skill();

        this
    }

    pub fn gain_xp(&mut self, gained_xp: f32) {
        self.accumulated_xp += gained_xp;

        if gained_xp != 0.0 {
            self.update_skill();
        }
    }

    pub fn get(&self) -> f32 {
        self.skill
    }

    fn update_skill(&mut self) {
        // This skill formula draws a curve that features a rapid growth early on,
        // which tapers off as more experience is gained.
        // Skill is bounded between MIN_SKILL and MAX_SKILL.
        // How quickly MAX_SKILL is approached is proportional to talent, which is in the range (0, inf]
        // A talent of 0 restricts skill to 0.
        self.skill = MIN_SKILL + (MAX_SKILL - MIN_SKILL) * std::f32::consts::E.powf(-1.0 / (self.talent * self.accumulated_xp))
    }
}

pub struct Skillset {
    traveling: Skill,
    wind_listening: Skill,
    trading: Skill,
}
