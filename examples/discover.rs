use anyhow::Result;
use futures::prelude::*;
use rupnp::ssdp::SearchTarget;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let devices = rupnp::discover(&SearchTarget::RootDevice, Duration::from_secs(3), None).await?;
    let mut devices = std::pin::pin!(devices);

    while let Some(device) = devices.next().await {
        let device = match device {
            Ok(device) => device,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        println!(
            "{} - {} @ {} (dst address: {}, interface #{})",
            device.device_type(),
            device.friendly_name(),
            device.url(),
            device.dst_ip().unwrap(),
            device.if_index().unwrap(),
        );
    }

    Ok(())
}
