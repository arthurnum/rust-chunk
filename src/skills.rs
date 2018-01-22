pub struct DeathRay {
    pub damage: f32,
    pub freq: f32 // milliseconds
}

impl DeathRay {
    pub fn apply(&self, target: &f32, elapsed: &f32) -> f32 {
        let k = elapsed/ self.freq;
        target - self.damage * k
    }
}
