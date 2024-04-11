use nvml_wrapper::Nvml;
fn main() {
    let nvml = Nvml::init().unwrap();
    // Get the first `Device` (GPU) in the system
    let device = nvml.device_by_index(0).unwrap();
    println!("device: {:?}", device);
    println!("pci_info: {:?}", device.pci_info().unwrap());
    println!(
        "performance_state: {:?}",
        device.performance_state().unwrap()
    );
    println!(
        "total_energy_consumption: {:?}",
        device.total_energy_consumption().unwrap()
    );
    println!("name: {:?}", device.name().unwrap());
    println!("fan_speed: {:?}", device.fan_speed(0).unwrap());
    println!("num_fans: {:?}", device.num_fans().unwrap());
    println!("encoder_util: {:?}", device.encoder_utilization().unwrap());
    println!("memory_info: {:?}", device.memory_info().unwrap()); // Currently 1.63/6.37 GB used on my system
}
