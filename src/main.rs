extern crate tiny_http;

use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{env, io};

use prometheus_client::encoding::text::{encode, Encode};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use vedirect::prelude::VedirectParser;

#[derive(Clone, Hash, PartialEq, Eq, Encode)]
struct Labels {
    // device name
    device: String,
}

const BIND_ADDR: &str = "0.0.0.0:9975";

fn main() {
    let args: Vec<String> = env::args().collect();
    let vedirect_sport: String = if args.len() > 1 {
        String::from(&args[1])
    } else {
        String::from("/dev/ttyS0")
    };

    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        std::process::exit(1);
    }));

    let device_name = "mppt";

    let mut registry = <Registry>::default();
    let battery_voltage = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_battery_voltage",
        "Main battery voltage in V",
        Box::new(battery_voltage.clone()),
    );
    let battery_current = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_battery_current",
        "Main battery current in A",
        Box::new(battery_current.clone()),
    );
    let battery_state = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_battery_state",
        "battery loading state",
        Box::new(battery_state.clone()),
    );
    let panel_voltage = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_panel_voltage",
        "Solar panel voltage in V",
        Box::new(panel_voltage.clone()),
    );
    let panel_power = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_panel_power",
        "Solar panel power in W",
        Box::new(panel_power.clone()),
    );
    let load_current = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "victron_load_current",
        "load current in A",
        Box::new(load_current.clone()),
    );
    let label = Labels {
        device: device_name.to_string(),
    };

    let server = Arc::new(tiny_http::Server::http(BIND_ADDR).unwrap());
    println!("Now listening on port 9975 and {}", vedirect_sport);

    let server = server.clone();

    let vedirect_reader_handle = thread::spawn(move || {
        println!("port: {}", vedirect_sport);

        let mut port = serialport::new(vedirect_sport, 19_200)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_secs(2))
            .open()
            .expect("Failed to open vedirect serial port");

        let mut serial_buf: Vec<u8> = vec![0; 1024];
        let mut parser = vedirect::Parser::new();
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    parser
                        .feed(&serial_buf[..t], |mppt: vedirect::MPPT| {
                            battery_voltage
                                .get_or_create(&label)
                                .set(mppt.channel1_voltage.into());
                            battery_current
                                .get_or_create(&label)
                                .set(mppt.battery_current.into());
                            battery_state
                                .get_or_create(&label)
                                .set((mppt.state_of_operation as u32).into());
                            panel_voltage
                                .get_or_create(&label)
                                .set(mppt.panel_voltage.into());
                            panel_power
                                .get_or_create(&label)
                                .set(mppt.panel_power.into());
                            load_current
                                .get_or_create(&label)
                                .set(mppt.load_current.into());
                        })
                        .unwrap();
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => eprintln!("timeout"),
                Err(e) => eprintln!("{:?}", e),
            }
            std::thread::sleep(Duration::from_millis(1000)); // vedirect transmits every second
        }
    });

    for rq in server.incoming_requests() {
        let mut buffer = vec![];
        encode(&mut buffer, &registry).unwrap();
        let response = tiny_http::Response::from_string(String::from_utf8(buffer).unwrap());
        let _ = rq.respond(response);
    }

    vedirect_reader_handle.join().unwrap();
}
