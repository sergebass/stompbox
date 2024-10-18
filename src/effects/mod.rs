pub mod no_op;
pub mod tremolo;
pub mod white_noise;

pub trait AudioEffect {
    fn name(&self) -> &str;
    fn process_sample(&mut self, input_sample: f32) -> f32;
}

pub fn new_audio_effect_by_name(
    client: &jack::Client,
    effect: &str,
) -> Box<dyn AudioEffect + Send> {
    match effect.to_lowercase().trim() {
        "tremolo" => Box::new(tremolo::TremoloEffect::new(client)),
        "whitenoise" => Box::new(white_noise::WhiteNoiseGenerator::new(random::default(42))),
        "noop" => Box::new(no_op::NoOpEffect {}),
        _ => Box::new(no_op::NoOpEffect {}),
    }
}
