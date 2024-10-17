use clap::Parser;
use jack;
use std::f32::consts::TAU;
use std::io::stdin;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Effect to enable
    #[arg(short, long)]
    effect: String,
}

// FIXME provide a method to work on whole buffers rather than single samples for efficiency?
// e.g. output.as_mut_slice(ps).copy_from_slice(state.silence.as_slice()); to copy entire slices

pub trait SampleProcessor {
    fn name(&self) -> &str;
    fn process_sample(&mut self, input_sample: f32) -> f32;
}

pub struct NoOpPassThroughProcessor {}

impl SampleProcessor for NoOpPassThroughProcessor {
    fn name(&self) -> &str {
        "No-Op"
    }

    fn process_sample(&mut self, input_sample: f32) -> f32 {
        input_sample // Leave the incoming sample unchanged.
    }
}

pub struct WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    random_source: T,
}

impl<T> WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    #[inline(always)]
    pub fn new(t: T) -> Self {
        Self { random_source: t }
    }
}

impl<T> SampleProcessor for WhiteNoiseGenerator<T>
where
    T: random::Source,
{
    fn name(&self) -> &str {
        "White Noise"
    }

    fn process_sample(&mut self, _input_sample: f32) -> f32 {
        self.random_source.read::<f32>()
    }
}

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

fn new_sample_processor_by_name(
    client: &jack::Client,
    effect: &str,
) -> Box<dyn SampleProcessor + Send> {
    match effect.to_lowercase().trim() {
        "tremolo" => Box::new(TremoloEffect::new(client)),
        "whitenoise" => Box::new(WhiteNoiseGenerator::new(random::default(42))),
        "noop" => Box::new(NoOpPassThroughProcessor {}),
        _ => Box::new(NoOpPassThroughProcessor {}),
    }
}

fn run_processing_loop(effect: String) {
    let (client, _status) =
        jack::Client::new("StompBox", jack::ClientOptions::NO_START_SERVER).unwrap();

    let input_port = client
        .register_port("Input", jack::AudioIn::default())
        .unwrap();

    let mut output_port = client
        .register_port("Output", jack::AudioOut::default())
        .unwrap();

    let mut sample_processor = new_sample_processor_by_name(&client, &effect);

    println!("Configured effect: {}", sample_processor.name());

    let process_handler = jack::contrib::ClosureProcessHandler::new(
        move |_client: &jack::Client, process_scope: &jack::ProcessScope| -> jack::Control {
            let input_buffer: &[f32] = input_port.as_slice(process_scope);
            let output_buffer: &mut [f32] = output_port.as_mut_slice(process_scope);

            let mut output_index = 0usize;

            for input_sample in input_buffer.iter() {
                output_buffer[output_index] = sample_processor.process_sample(*input_sample);

                output_index += 1;
            }

            jack::Control::Continue
        },
    );

    let _active_client = client.activate_async((), process_handler).unwrap();

    loop {
        let mut input_text = String::new();
        stdin().read_line(&mut input_text).expect("error");
    }
}

fn main() {
    let args = Args::parse();

    println!("Chosen effect: {}", args.effect);

    println!("StompBox is starting. Make sure to establish the right Jack connections (e.g using qjackctl).");

    run_processing_loop(args.effect);
}
