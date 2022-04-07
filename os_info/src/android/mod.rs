use log::trace;

use crate::{Bitness, Info, Type, system,Version};

pub fn current_platform() -> Info {
    trace!("android::current_platform is called");

    let mut info = Info::with_type(Type::Android);
    trace!("Returning {:?}", info);
    info.version = Version::from_string(system::getprop("ro.build.version.release").unwrap_or("".to_string()));
    info.edition = system::getprop("ro.build.version.security_patch");
    info.model = system::getprop("ro.product.system.device");
    if info.model.is_none() {
        info.model = system::getprop("ro.product.model");
    }
    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Android, version.os_type());
    }
}
