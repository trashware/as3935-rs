use as3935::interface::i2c::I2cAddress;
use as3935::{
    Event, HeadOfStormDistance, InterfaceSelection, ListeningParameters, SensorPlacing,
    SignalVerificationThreshold, AS3935,
};
use chrono::Utc;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use simple_signal::{set_handler, Signal};
use std::sync::mpsc::channel;

fn main() {
    simple_logger::init().unwrap();

    println!("Initializing…");

    let gpio = Gpio::new().unwrap();

    let mut as3935 = AS3935::new(
        InterfaceSelection::I2c(I2c::with_bus(1).unwrap(), I2cAddress::default()),
        gpio.get(24).unwrap().into_input(),
    )
    .unwrap();

    println!("Starting to listen…");

    let events = as3935
        .listen(
            ListeningParameters::default()
                .with_sensor_placing(SensorPlacing::Outdoor)
                .with_signal_verification_threshold(SignalVerificationThreshold::new(5).unwrap()),
        )
        .unwrap();

    println!("Listening for events…");

    std::thread::spawn(move || {
        for event in events {
            println!(
                "[{}] {}",
                Utc::now().to_rfc3339(),
                match event {
                    Event::Lightning(lightning) => format!(
                        "Lightning detected: {}.",
                        match lightning {
                            HeadOfStormDistance::Kilometers(km) => format!("{} km", km),
                            HeadOfStormDistance::OutOfRange => String::from("out of range"),
                            HeadOfStormDistance::Overhead => String::from("overhead"),
                        }
                    ),
                    Event::Noise => String::from("Noise detected."),
                    Event::Disturbance => String::from("Disturber detected."),
                }
            )
        }
    });

    let (tx, rx) = channel::<()>();

    set_handler(&[Signal::Term, Signal::Int], move |_signals| {
        tx.send(()).unwrap();
    });

    rx.recv().unwrap();

    println!("Terminating…");
    as3935.terminate().unwrap();

    println!("Terminated.");
}
