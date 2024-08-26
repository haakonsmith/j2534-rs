use std::io;
use std::fs;

use winreg::{enums::*, RegKey};
use pe_parser::pe;

use crate::drivers::Driver;

const PASSTHRU_REGPATHS: [&'static str; 2] = [
    "SOFTWARE\\WOW6432Node\\PassThruSupport.04.04",
    "SOFTWARE\\PassThruSupport.04.04",
];

const MACHINE_TYPE: u16 = if std::mem::size_of::<usize>() == 4 {0x8664} else {0x14c};

/// Returns a list of all installed PassThru drivers
pub fn list_drivers() -> io::Result<Vec<Driver>> {
    let mut listings = Vec::new();
    // Loop over all the possible registry paths
    for regpath in &PASSTHRU_REGPATHS{
        let passthru = match RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(regpath)
        {
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

            let dll: Vec<u8> = fs::read(&path)?;
            let pe: Result<pe::PortableExecutable, pe_parser::Error> = pe::parse_portable_executable(dll.as_slice());

            // Only add drivers that are compatible with the current machine
            if pe.is_ok_and(|pe: pe::PortableExecutable| pe.coff.machine == MACHINE_TYPE) {
                listings.push(Driver {
                    name: device_name,
                    vendor,
                    path,
                });
            }
        }
    }

    Ok(listings)
}