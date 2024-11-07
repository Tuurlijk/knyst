use anyhow::Result;
use knyst::controller::schedule_bundle;
use knyst::graph;
#[allow(unused)]
use knyst::{
    audio_backend::{CpalBackend, CpalBackendOptions, JackBackend},
    controller::print_error_handler,
    prelude::*,
};

/// Initializes the audio backend, sets up the `KnystSphere`, and configures a wavetable oscillator to output audio.
///
/// The function performs the following steps:
/// 1. Initializes the audio backend and starts a `KnystSphere`.
/// 2. Sets the active sphere for audio processing.
/// 3. Creates an owned wavetable oscillator set at 110 Hz.
/// 4. Wait 3 seconds
/// 5. Outputs the oscillator to the left and right channels at 30% volume each.
/// 6. Waits for user to press ENTER.
///
fn main() -> Result<()> {
    let _backend = setup();

    let node0 = wavetable_oscillator_owned(Wavetable::sine()).freq(110.);

    schedule_bundle(graph::Time::Seconds(Seconds::new(3, 0)), || {
        graph_output(0, node0 * 0.3);
        graph_output(1, node0 * 0.3);
    });

    println!("Playing a sine wave at 110 Hz at an amplitude of 0.3, after 3 seconds");
    println!("Press [ENTER] to exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}

/// Initializes the audio backend and starts a `KnystSphere` for audio processing.
/// Start with an automatic helper thread for scheduling changes and managing resources.
/// If you want to manage the `Controller` yourself, use `start_return_controller`.
///
/// The backend is returned here because it would otherwise be dropped at the end of setup()
///
/// # Returns
///
/// A tuple containing:
/// - `impl AudioBackend`: The initialized audio backend, either CPAL or JACK.
/// - `Option<SphereId>`: The optional unique identifier for the created `KnystSphere`.
fn setup() -> impl AudioBackend {
    let mut backend =
        CpalBackend::new(CpalBackendOptions::default()).expect("Unable to connect to CPAL backend");
    // Uncomment the line below and comment the line above to use the JACK backend instead
    // let mut backend = JackBackend::new("Knyst<3JACK").expect("Unable to start JACK backend");

    let _sphere_id = KnystSphere::start(
        &mut backend,
        SphereSettings {
            ..Default::default()
        },
        print_error_handler,
    )
    .ok();
    backend
}
