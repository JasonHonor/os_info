use std::process::Command;
use std::str;

use log::{error, trace};

use crate::{bitness, uname::uname, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("illumos::current_platform is called");

    let version = uname()
        .map(Version::from_string)
        .unwrap_or_else(|| Version::Unknown);

    let info = Info {
        os_type: get_os(),
        version,
        bitness: bitness::get(),
        ..Default::default()
    };

    trace!("Returning {:?}", info);
    info
}

fn get_os() -> Type {
    let os = Command::new("uname")
        .arg("-o")
        .output()
        .expect("Failed to get OS");

    match str::from_utf8(&os.stdout) {
        Ok("illumos") => {
            Type::Illumos
        },
        Ok(_) => {
            Type::Unknown
        },
        Err(_) => {
            Type::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Illumos, version.os_type());
    }
}
