cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use windows::list_drivers as list;
    } else {
        pub fn list() -> io::Result<Vec<Driver>> {
            Ok(Vec::new())
        }
    }
}

use std::fmt;

bitflags! {
    pub struct ProtocolFlag: u32 {
        const NONE      = 0b00000000;
        const GM_UART   = 0b00000001;
        const ISO9141   = 0b00000010;
        const ISO14230  = 0b00000100;
        const CAN       = 0b00001000;
        const ISO15765  = 0b00010000;
        const J1850PWM  = 0b00100000;
        const J1850VPW  = 0b01000000;
        const J2610     = 0b10000000;
    }
}

impl fmt::Display for ProtocolFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut protocols: Vec<&str> = Vec::new();
        if self.contains(ProtocolFlag::GM_UART) {
            protocols.push("GM_UART");
        }
        if self.contains(ProtocolFlag::ISO9141) {
            protocols.push("ISO9141");
        }
        if self.contains(ProtocolFlag::ISO14230) {
            protocols.push("ISO14230");
        }
        if self.contains(ProtocolFlag::CAN) {
            protocols.push("CAN");
        }
        if self.contains(ProtocolFlag::ISO15765) {
            protocols.push("ISO15765");
        }
        if self.contains(ProtocolFlag::J1850PWM) {
            protocols.push("J1850PWM");
        }
        if self.contains(ProtocolFlag::J1850VPW) {
            protocols.push("J1850VPW");
        }
        if self.contains(ProtocolFlag::J2610) {
            protocols.push("J2610");
        }
        write!(f, "{}", protocols.join(","))
    }
}

/// Information about an installed PassThru driver
#[derive(Debug)]
pub struct Driver {
    pub name: String,
    pub vendor: String,
    pub path: String,
    pub protocols: ProtocolFlag,
}
