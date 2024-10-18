pub mod no_op;
pub mod tremolo;
pub mod white_noise;

pub trait SampleProcessor {
    fn name(&self) -> &str;
    fn process_sample(&mut self, input_sample: f32) -> f32;
}

pub fn new_sample_processor_by_name(
    client: &jack::Client,
    effect: &str,
) -> Box<dyn SampleProcessor + Send> {
    match effect.to_lowercase().trim() {
        "tremolo" => Box::new(tremolo::TremoloEffect::new(client)),
        "whitenoise" => Box::new(white_noise::WhiteNoiseGenerator::new(random::default(42))),
        "noop" => Box::new(no_op::NoOpPassThroughProcessor {}),
        _ => Box::new(no_op::NoOpPassThroughProcessor {}),
    }
}
