use std::{collections::HashMap, fs, process::Command};
use common::{DeviceConfig};

fn main() -> std::io::Result<()> {
    let cfg_file = fs::read_to_string("hidl.toml")?;
    let cfg: HashMap<String, DeviceConfig> = toml::from_str(&cfg_file).expect("failed to parse config file `hidl.toml`");
    let flash = cfg["lvb"].flash.clone().unwrap();

    println!("{:?}", flash.write_flash_args("."));
    let esptool = Command::new("esptool")
        .args(flash.write_flash_args("."))
        .output()
        .unwrap();

    println!("status: {}", esptool.status);
    println!("stdout: {}", String::from_utf8(esptool.stdout).unwrap());
    println!("stderr: {}", String::from_utf8(esptool.stderr).unwrap());

    Ok(())
}
