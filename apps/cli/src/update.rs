use anyhow::{Context, Result};
use self_update::backends::github::Update;
use std::env;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn release_asset_identifier() -> Result<&'static str> {
    Ok("linux-x86_64")
}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn release_asset_identifier() -> Result<&'static str> {
    Ok("macos-aarch64")
}
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn release_asset_identifier() -> Result<&'static str> {
    Ok("windows-x86_64")
}
#[cfg(not(any(
    all(target_os = "linux", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "windows", target_arch = "x86_64")
)))]
fn release_asset_identifier() -> Result<&'static str> {
    anyhow::bail!("OpenPixel has no release package for this operating system and architecture")
}

/// Update this executable and its sibling from the matching GitHub release archive.
pub fn update_binaries(
    binary_name: &str,
    companion_name: &str,
    current_version: &str,
    force: bool,
    quiet: bool,
) -> Result<String> {
    let current_exe = env::current_exe().context("could not locate the running executable")?;
    let install_dir = current_exe
        .parent()
        .context("the running executable has no parent directory")?;
    let companion_path = install_dir.join(format!("{companion_name}{}", env::consts::EXE_SUFFIX));
    let asset_identifier = release_asset_identifier()?;
    let selected_version = if force { "0.0.0" } else { current_version };
    let mut updated_version = None;

    for (name, install_path) in [(binary_name, current_exe), (companion_name, companion_path)] {
        let status = Update::configure()
            .repo_owner("yaso09")
            .repo_name("openpixel")
            .bin_name(name)
            .bin_install_path(install_path)
            .bin_path_in_archive(format!("{name}{}", env::consts::EXE_SUFFIX))
            .asset_identifier(asset_identifier)
            .current_version(selected_version)
            .no_confirm(true)
            .show_output(!quiet)
            .show_download_progress(!quiet)
            .build()
            .context("could not configure the OpenPixel updater")?
            .update()
            .context("could not install the latest OpenPixel release")?;
        if status.is_updated() {
            updated_version = Some(status.version().to_string());
        }
    }

    if let Some(version) = updated_version {
        Ok(format!(
            "Updated OpenPixel to {version}. Restart the application to use the new version."
        ))
    } else {
        Ok(format!(
            "OpenPixel {current_version} is already up to date."
        ))
    }
}
