use itertools::Itertools;
use serialport::available_ports;

fn main() {
    let a = if let Ok(ports) = available_ports() {
        ports.into_iter().map(|p| p.port_name).sorted().collect()
    } else {
        Vec::new()
    };

    println!("{a:?}");
}
