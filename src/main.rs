use anyhow::{Result, anyhow};
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("color-scheme")
        .output()?;

    let mode = if output.status.success() {
        String::from_utf8(output.stdout)?.lines().next().unwrap().to_owned()
    } else {
        return Err(anyhow!("Erreur get color-scheme: {}", output.status));
    };

    let flip = if mode == "'prefer-dark'" { "prefer-light" } else { "prefer-dark" };

    Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.interface")
        .arg("color-scheme")
        .arg(flip)
        .output()?;
    Ok(())
}
