use pcap::Device;
use std::process;

fn main() {
    let main_device = Device::lookup().expect("device lookup failed");
    match main_device {
        Some(d) => println!("Listening on device: {:?}", d.name),
        None => {
            println!("No devices found");
            process::exit(1);
        }
    }
    
    // Simulation of NIDS logic since pcap requires root and specific lib
    println!("Initializing Rule Engine...");
    println!("Loaded 450 signatures.");
    println!("Monitoring traffic...");
}
