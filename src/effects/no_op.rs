use crate::effects::SampleProcessor;

pub struct NoOpPassThroughProcessor {}

impl SampleProcessor for NoOpPassThroughProcessor {
    fn name(&self) -> &str {
        "No-Op"
    }

    fn process_sample(&mut self, input_sample: f32) -> f32 {
        input_sample // Leave the incoming sample unchanged.
    }
}
