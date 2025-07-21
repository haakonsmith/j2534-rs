use std::fs;
use std::io;

use pe_parser::pe;
use winreg::{enums::*, RegKey};

use crate::drivers::{Driver, ProtocolFlag};

const PASSTHRU_REGPATHS: [&'static str; 2] = [
    "SOFTWARE\\WOW6432Node\\PassThruSupport.04.04",
    "SOFTWARE\\PassThruSupport.04.04",
];

/// Returns a list of all installed PassThru drivers
pub fn list_drivers() -> io::Result<Vec<Driver>> {
    let mut listings = Vec::new();
    // Loop over all the possible registry paths
    for regpath in &PASSTHRU_REGPATHS {
        let passthru = match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(regpath) {
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            other => other,
        }?;

        for name in passthru.enum_keys() {
            let name: String = name?;
            let key: RegKey = passthru.open_subkey(name)?;

            let device_name: String = key.get_value("Name")?;
            let vendor: String = key.get_value("Vendor")?;
            let path: String = key.get_value("FunctionLibrary")?;
            let mut protocols = ProtocolFlag::NONE;

            for (name, _value) in key
                .enum_values()
                .map(|x: Result<(String, winreg::RegValue), io::Error>| x.unwrap())
            {
                if name.starts_with("GM_UART") {
                    protocols |= ProtocolFlag::GM_UART;
                } else if name.starts_with("ISO9141") {
                    protocols |= ProtocolFlag::ISO9141;
                } else if name.starts_with("ISO14230") {
                    protocols |= ProtocolFlag::ISO14230;
                } else if name.starts_with("CAN") {
                    protocols |= ProtocolFlag::CAN;
                } else if name.starts_with("ISO15765") {
                    protocols |= ProtocolFlag::ISO15765;
                } else if name.starts_with("J1850PWM") {
                    protocols |= ProtocolFlag::J1850PWM;
                } else if name.starts_with("J1850VPW") {
                    protocols |= ProtocolFlag::J1850VPW;
                } else if name.starts_with("J2610") {
                    protocols |= ProtocolFlag::J2610;
                }
            }

            listings.push(Driver {
                name: device_name,
                vendor,
                path,
                protocols,
            });
        }
    }

    Ok(listings)
}
