// use random::Source;
use jack;
use std::f32::consts::TAU;
use std::io::stdin;

fn run_processing_loop() {
    let (client, _status) =
        jack::Client::new("StompBox", jack::ClientOptions::NO_START_SERVER).unwrap();

    let input_port = client
        .register_port("Input", jack::AudioIn::default())
        .unwrap();

    let mut output_port = client
        .register_port("Output", jack::AudioOut::default())
        .unwrap();

    let mut time: usize = 0;

    let warble_frequency_hz: f32 = 3.0;

    let mut sample_rate = client.sample_rate();

    // let mut random_source = random::default(42);

    let angular_frequency = warble_frequency_hz * TAU / sample_rate as f32;

    let mut audio_processor = move |input_sample: &f32| -> f32 {
        let level_change_factor = (time as f32 * angular_frequency).sin();

        time += 1;

        *input_sample * level_change_factor

        // Generate some white noise for testing
        // random_source.read::<f32>()
    };

    let handler = jack::contrib::ClosureProcessHandler::new(
        move |client: &jack::Client, process_scope: &jack::ProcessScope| -> jack::Control {
            if sample_rate != client.sample_rate() {
                sample_rate = client.sample_rate();
                println!("Sample rate changed to {sample_rate}");
            }

            let input_buffer = input_port.as_slice(process_scope);
            let output_buffer = output_port.as_mut_slice(process_scope);

            let mut output_index: usize = 0;

            for input_sample in input_buffer.iter() {

                output_buffer[output_index] = audio_processor(input_sample);

                output_index += 1;
            }

            jack::Control::Continue
        },
    );

    let _active_client = client.activate_async((), handler).unwrap();

    loop {
        let mut input_text = String::new();
        stdin().read_line(&mut input_text).expect("error");
    }
}

fn main() {
    println!("StompBox is starting. Make sure to establish the right Jack connections (e.g using qjackctl).");

    // TODO parse command line options here
    run_processing_loop();
}
