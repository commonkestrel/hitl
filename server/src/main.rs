use anyhow::{Context, bail};
use common::{BitstreamConfig, DeviceConfig};
use smol::fs;
use smol::process::Command;
use std::collections::HashMap;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let cfg_file = fs::read_to_string("hitl.toml")
            .await
            .context("Failed to read config from `hitl.toml`")?;
        let cfg: HashMap<String, DeviceConfig> =
            toml::from_str(&cfg_file).context("Failed to parse config")?;

        let execute_out = execute(".", &cfg["lvb"]).await?;
        println!(
            "bitstream bytes copied: {:?}\nesptool stderr: {:?}",
            execute_out.bitstream_bytes, execute_out.flash_stderr
        );

        Ok(())
    })
}

struct ExecuteOut {
    bitstream_bytes: Option<u64>,
    flash_stderr: Option<String>,
}

async fn execute<P: AsRef<Path>>(root: P, device: &DeviceConfig) -> anyhow::Result<ExecuteOut> {
    let bitstream_bytes = match &device.bitstream {
        Some(BitstreamConfig::Volume { bin, volume }) => Some(
            fs::copy(root.as_ref().join(bin), volume)
                .await
                .context("Failed to copy bitstream to volume")?,
        ),
        Some(BitstreamConfig::Serial { bin: _, sink: _, baud: _ }) => unimplemented!(),
        None => None,
    };

    let flash_stderr = if let Some(ref flash) = device.flash {
        let esptool = Command::new("esptool")
            .args(flash.write_flash_args(root))
            .output()
            .await
            .context("Failed to run the `esptool` command")?;

        if !esptool.status.success() {
            bail!("`esptool` failed with exit status {}", esptool.status);
        }

        Some(String::from_utf8(esptool.stderr).context("`esptool` returned invalid UTF-8")?)
    } else {
        None
    };

    Ok(ExecuteOut {
        bitstream_bytes,
        flash_stderr,
    })
}
