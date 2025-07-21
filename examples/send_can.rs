use j2534::PassThruMsg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a list of interfaces
    let device = match j2534::drivers::list()?.into_iter().next() {
        Some(device) => device,
        None => {
            println!("No J2534 interfaces found");
            return Ok(());
        }
    };

    println!("Opening interface '{}'", device.name);
    let iface = j2534::Interface::new(&device.path)?;

    // Open any connected device
    let device = iface.open_any()?;

    // Get version information
    let version_info = device.read_version()?;
    println!("{:#?}", version_info);

    // Open a CAN channel with a baudrate of 500k.
    let channel = device.connect(j2534::Protocol::CAN, j2534::ConnectFlags::NONE, 500000)?;

    let message = PassThruMsg::new_can(8, &[0, 1, 2, 3]);

    // try forever to write message
    while channel.write(&mut [message], 100)? == 0 {}

    Ok(())
}
