use std::{path::{Path, PathBuf}, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceConfig {
    pub flash: Option<FlashConfig>,
    pub bitstream: Option<BitstreamConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BitstreamConfig {
    Volume{bin: String, volume: String},
    Serial{bin: String, sink: String, baud: String},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashConfig {
    chip: Option<Chip>,
    baud: usize,
    #[serde(flatten)]
    port: Option<Port>,
    #[serde(default)]
    before: BeforeConnectMode,
    #[serde(default)]
    after: AfterOperationMode,
    flash_freq: FlashFreq,
    flash_mode: FlashMode,
    #[serde(default)]
    flash_size: FlashSize,
    blobs: Vec<Blob>,
}

impl FlashConfig {
    pub fn write_flash_args<P: AsRef<Path>>(&self, root: P) -> Vec<String> {
        let mut args = Vec::new();

        // Target chip type
        if let Some(chip) = self.chip {
            args.push("--chip".to_owned());
            args.push(chip.to_string());
        }

        // Serial port baud rate for this device
        args.push("--baud".to_owned());
        args.push(self.baud.to_string());

        match self.port.clone() {
            Some(Port::Port(serial)) => {
                args.push("--port".to_owned());
                args.push(serial);
            }
            Some(Port::PortFilter(filter)) => {
                args.push("--port-filter".to_owned());
                args.push(filter);
            }
            None => {},
        };
        
        args.push("--before".to_owned());
        args.push(self.before.to_string());

        args.push("--after".to_owned());
        args.push(self.after.to_string());

        args.push("write-flash".to_owned());

        args.push("--flash-mode".to_owned());
        args.push(self.flash_mode.to_string());

        args.push("--flash-size".to_owned());
        args.push(self.flash_size.to_string());

        args.push("--flash-freq".to_owned());
        args.push(self.flash_freq.to_string());

        for blob in self.blobs.iter() {
            blob.append_to_arguments(&mut args, &root);
        }

        return args;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Port {
    Port(String),
    PortFilter(String)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    addr: u32,
    file: PathBuf,
}

impl Blob {
    fn append_to_arguments<P: AsRef<Path>>(&self, arguments: &mut Vec<String>, root: P) {
        arguments.push(self.addr.to_string());
        arguments.push(root.as_ref().join(&self.file).to_string_lossy().to_string());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chip {
    Esp32,
    Esp32C2,
    Esp32C3,
    Esp32C5,
    Esp32C6,
    Esp32C61,
    Esp32E22,
    Esp32H2,
    Esp32H21,
    Esp32H4,
    Esp32P4,
    Esp32S2,
    Esp32S3,
    Esp32S31,
    Esp8266,
}

impl ToString for Chip {
    fn to_string(&self) -> String {
        match self {
            Chip::Esp32 => "esp32".to_owned(),
            Chip::Esp32C2 => "esp32c2".to_owned(),
            Chip::Esp32C3 => "esp32c3".to_owned(),
            Chip::Esp32C5 => "esp32c5".to_owned(),
            Chip::Esp32C6 => "esp32c6".to_owned(),
            Chip::Esp32C61 => "esp32c61".to_owned(),
            Chip::Esp32E22 => "esp32e22".to_owned(),
            Chip::Esp32H2 => "esp32h2".to_owned(),
            Chip::Esp32H21 => "esp32h21".to_owned(),
            Chip::Esp32H4 => "esp32h4".to_owned(),
            Chip::Esp32P4 => "esp32p4".to_owned(),
            Chip::Esp32S2 => "esp32s2".to_owned(),
            Chip::Esp32S3 => "esp32s3".to_owned(),
            Chip::Esp32S31 => "esp32s31".to_owned(),
            Chip::Esp8266 => "esp8266".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BeforeConnectMode {
    DefaultReset,
    UsbReset,
    NoReset,
    NoResetNoSync,
}

impl FromStr for BeforeConnectMode {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default-reset" => Ok(BeforeConnectMode::DefaultReset),
            "usb-reset" => Ok(BeforeConnectMode::UsbReset),
            "no-reset" => Ok(BeforeConnectMode::NoReset),
            "no-reset-no-sync" => Ok(BeforeConnectMode::NoResetNoSync),
            _ => Err(()),
        }
    }
}

impl ToString for BeforeConnectMode {
    fn to_string(&self) -> String {
        match self {
            BeforeConnectMode::DefaultReset => "default-reset".to_owned(),
            BeforeConnectMode::UsbReset => "usb-reset".to_owned(),
            BeforeConnectMode::NoReset => "no-reset".to_owned(),
            BeforeConnectMode::NoResetNoSync => "no-reset-no-sync".to_owned(),
        }
    }
}

impl Default for BeforeConnectMode {
    fn default() -> Self {
        return Self::DefaultReset;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AfterOperationMode {
    HardReset,
    SoftReset,
    NoReset,
    NoResetStub,
    WatchdogReset,
}

impl FromStr for AfterOperationMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hard-reset" => Ok(AfterOperationMode::HardReset),
            "soft-reset" => Ok(AfterOperationMode::SoftReset),
            "no-reset" => Ok(AfterOperationMode::NoReset),
            "no-reset-stub" => Ok(AfterOperationMode::NoResetStub),
            "watchdog-reset" => Ok(AfterOperationMode::WatchdogReset),
            _ => Err(()),
        }
    }
}

impl ToString for AfterOperationMode {
    fn to_string(&self) -> String {
        match self {
            AfterOperationMode::HardReset => "hard-reset".to_owned(),
            AfterOperationMode::SoftReset => "soft-reset".to_owned(),
            AfterOperationMode::NoReset => "no-reset".to_owned(),
            AfterOperationMode::NoResetStub => "no-reset-stub".to_owned(),
            AfterOperationMode::WatchdogReset => "watchdog-reset".to_owned(),
        }
    }
}

impl Default for AfterOperationMode {
    fn default() -> Self {
        return Self::HardReset;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlashFreq {
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "80m")]
    M80,
    #[serde(rename = "64m")]
    M64,
    #[serde(rename = "60m")]
    M60,
    #[serde(rename = "48m")]
    M48,
    #[serde(rename = "40m")]
    M40,
    #[serde(rename = "32m")]
    M32,
    #[serde(rename = "30m")]
    M30,
    #[serde(rename = "26m")]
    M26,
    #[serde(rename = "24m")]
    M24,
    #[serde(rename = "20m")]
    M20,
    #[serde(rename = "16m")]
    M16,
    #[serde(rename = "15m")]
    M15,
    #[serde(rename = "12m")]
    M12,
}

impl FromStr for FlashFreq {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "keep" => Ok(FlashFreq::Keep),
            "80m" => Ok(FlashFreq::M80),
            "64m" => Ok(FlashFreq::M64),
            "60m" => Ok(FlashFreq::M60),
            "48m" => Ok(FlashFreq::M48),
            "40m" => Ok(FlashFreq::M40),
            "32m" => Ok(FlashFreq::M32),
            "30m" => Ok(FlashFreq::M30),
            "26m" => Ok(FlashFreq::M26),
            "24m" => Ok(FlashFreq::M24),
            "20m" => Ok(FlashFreq::M20),
            "16m" => Ok(FlashFreq::M16),
            "15m" => Ok(FlashFreq::M15),
            "12m" => Ok(FlashFreq::M12),
            _ => Err(()),
        }
    }
}

impl ToString for FlashFreq {
    fn to_string(&self) -> String {
        match self {
            FlashFreq::Keep => "keep".to_owned(),
            FlashFreq::M80 => "80m".to_owned(),
            FlashFreq::M64 => "64m".to_owned(),
            FlashFreq::M60 => "60m".to_owned(),
            FlashFreq::M48 => "48m".to_owned(),
            FlashFreq::M40 => "40m".to_owned(),
            FlashFreq::M32 => "32m".to_owned(),
            FlashFreq::M30 => "30m".to_owned(),
            FlashFreq::M26 => "26m".to_owned(),
            FlashFreq::M24 => "24m".to_owned(),
            FlashFreq::M20 => "20m".to_owned(),
            FlashFreq::M16 => "16m".to_owned(),
            FlashFreq::M15 => "15m".to_owned(),
            FlashFreq::M12 => "12m".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlashMode {
    Keep,
    Qio,
    Qout,
    Dio,
    Dout,
}

impl FromStr for FlashMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "keep" => Ok(FlashMode::Keep),
            "qio" => Ok(FlashMode::Qio),
            "qout" => Ok(FlashMode::Qout),
            "dio" => Ok(FlashMode::Dio),
            "dout" => Ok(FlashMode::Dout),
            _ => Err(()),
        }
    }
}

impl ToString for FlashMode {
    fn to_string(&self) -> String {
        match self {
            FlashMode::Keep => "keep".to_owned(),
            FlashMode::Qio => "qio".to_owned(),
            FlashMode::Qout => "qout".to_owned(),
            FlashMode::Dio => "dio".to_owned(),
            FlashMode::Dout => "dout".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlashSize {
    #[serde(rename = "detect")]
    Detect,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "256KB")]
    KB256,
    #[serde(rename = "512KB")]
    KB512,
    #[serde(rename = "1MB")]
    MB1,
    #[serde(rename = "2MB")]
    MB2,
    #[serde(rename = "2MB-c1")]
    MB2C1,
    #[serde(rename = "4MB")]
    MB4,
    #[serde(rename = "4MB-c1")]
    MB4C1,
    #[serde(rename = "8MB")]
    MB8,
    #[serde(rename = "16MB")]
    MB16,
    #[serde(rename = "32MB")]
    MB32,
    #[serde(rename = "64MB")]
    MB64,
    #[serde(rename = "128MB")]
    MB128,
}

impl FromStr for FlashSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "detect" => Ok(FlashSize::Detect),
            "keep" => Ok(FlashSize::Keep),
            "256KB" => Ok(FlashSize::KB256),
            "512KB" => Ok(FlashSize::KB512),
            "1MB" => Ok(FlashSize::MB1),
            "2MB" => Ok(FlashSize::MB2),
            "2MB-c1" => Ok(FlashSize::MB2C1),
            "4MB" => Ok(FlashSize::MB4),
            "4MB-c1" => Ok(FlashSize::MB4C1),
            "8MB" => Ok(FlashSize::MB8),
            "16MB" => Ok(FlashSize::MB16),
            "32MB" => Ok(FlashSize::MB32),
            "64MB" => Ok(FlashSize::MB64),
            "128MB" => Ok(FlashSize::MB128),
            _ => Err(()),
        }
    }
}

impl ToString for FlashSize {
    fn to_string(&self) -> String {
        match self {
            FlashSize::Detect => "detect".to_owned(),
            FlashSize::Keep => "keep".to_owned(),
            FlashSize::KB256 => "256KB".to_owned(),
            FlashSize::KB512 => "512KB".to_owned(),
            FlashSize::MB1 => "1MB".to_owned(),
            FlashSize::MB2 => "2MB".to_owned(),
            FlashSize::MB2C1 => "2MB-c1".to_owned(),
            FlashSize::MB4 => "4MB".to_owned(),
            FlashSize::MB4C1 => "4MB-c1".to_owned(),
            FlashSize::MB8 => "8MB".to_owned(),
            FlashSize::MB16 => "16MB".to_owned(),
            FlashSize::MB32 => "32MB".to_owned(),
            FlashSize::MB64 => "64MB".to_owned(),
            FlashSize::MB128 => "128MB".to_owned(),
        }
    }
}

impl Default for FlashSize {
    fn default() -> Self {
        return Self::Detect;
    }
}
