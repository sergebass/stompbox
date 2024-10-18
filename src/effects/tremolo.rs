use crate::effects::SampleProcessor;
use jack;
use std::f32::consts::TAU;

pub struct TremoloEffect {
    angular_frequency: f32,
    time: usize,
}

impl TremoloEffect {
    #[inline(always)]
    pub fn new(client: &jack::Client) -> Self {
        let sample_rate = client.sample_rate();
        let warble_frequency_hz = 3.0;

        Self {
            angular_frequency: warble_frequency_hz * TAU / sample_rate as f32,
            time: 0,
        }
    }
}

impl SampleProcessor for TremoloEffect {
    fn name(&self) -> &str {
        "Tremolo/Warble"
    }

    fn process_sample(&mut self, input_sample: f32) -> f32 {
        let level_change_factor = (self.time as f32 * self.angular_frequency).sin();

        self.time += 1;

        input_sample * level_change_factor
    }
}
