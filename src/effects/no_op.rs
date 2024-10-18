use crate::effects::AudioEffect;

pub struct NoOpEffect {}

impl AudioEffect for NoOpEffect {
    fn name(&self) -> &str {
        "No-Op"
    }

    fn process_sample(&mut self, input_sample: f32) -> f32 {
        input_sample // Leave the incoming sample unchanged.
    }
}
