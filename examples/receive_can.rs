use j2534::{PassThruMsg, Protocol};

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

    // Create a filter allowing all messages to be received. This works because we are passing it as a mask and it's masking out ALL the bits
    let empty_filter = PassThruMsg::new_can(0, &[]);

    channel.start_message_filter(
        j2534::FilterType::Pass,
        Some(&empty_filter),
        Some(&empty_filter),
        None,
    )?;

    let mut messages = [PassThruMsg::new(Protocol::CAN); 32];

    // Read up to 32 messages. Returns the number of messages actually read before timeout
    let count = channel.read(&mut messages, 1000)?;
    for msg in &messages[..count] {
        if let Some((id, data)) = msg.can_message() {
            println!("{:X}: {:X?}", id, data);
        }
    }
    Ok(())
}
