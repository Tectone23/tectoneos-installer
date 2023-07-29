use std::process::Command;

pub async fn check_adb() -> Result<(), ()> {
    return match Command::new("adb").arg("devices").output() {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    };
}
pub async fn check_fastboot() -> Result<(), ()> {
    return match Command::new("fastboot").arg("devices").output() {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    };
}
