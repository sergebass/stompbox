use clap::Parser;
use jack;
use std::io::stdin;

mod effects;

#[derive(clap::Parser, Debug)]
struct Args {
    /// Effect to enable
    #[arg(short, long)]
    effect: String,
}

fn run_processing_loop(effect_name: &str) {
    let (client, _status) = jack::Client::new(
        &format!("StompBox ({effect_name})"),
        jack::ClientOptions::NO_START_SERVER,
    )
    .unwrap();

    let input_port = client
        .register_port("Input", jack::AudioIn::default())
        .unwrap();

    let mut output_port = client
        .register_port("Output", jack::AudioOut::default())
        .unwrap();

    let mut sample_processor = effects::new_audio_effect_by_name(&client, &effect_name);

    println!("Configured effect: {}", sample_processor.name());

    let process_handler = jack::contrib::ClosureProcessHandler::new(
        move |_client: &jack::Client, process_scope: &jack::ProcessScope| -> jack::Control {
            let input_buffer: &[f32] = input_port.as_slice(process_scope);
            let output_buffer: &mut [f32] = output_port.as_mut_slice(process_scope);

            let mut output_index: usize = 0;

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

    run_processing_loop(&args.effect);
}
