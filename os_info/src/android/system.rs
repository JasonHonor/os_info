use std::process::Command;
use log::error;

pub fn getprop(item:&str) -> Option<String> {
    Command::new("getprop")
        .arg(item)
        .output()
        .map_err(|e| {
            error!("Failed to invoke 'getprop': {:?}", e);
        })
        .ok()
        .and_then(|out| {
            if out.status.success() {
                let result = String::from_utf8_lossy(&out.stdout).replace("\n","").replace("\r","").trim_end().to_owned();
                if result.is_empty() {
                    None
                }else {
                    Some(result)
                }
            } else {
                log::error!("'getprop' invocation error: {:?}", out);
                None
            }
        })
}
