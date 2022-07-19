
use std::error::Error;
use std::os::unix::fs::symlink;
use std::path::Path;

use clap::ArgMatches;
use simple_error::*;
use simplelog::*;

#[cfg(target_os = "macos")]
use crate::macos::*;

#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "linux")]
use crate::linux::*;

use crate::escalate::*;

pub fn get_alias(args: &ArgMatches) -> Option<String> {
    match args.value_of("str") {
        None => None,
        Some(str) => {
            match str {
                "oldrel" | "oldrel/1" => Some("oldrel".to_string()),
                "release" | "devel" | "next" => Some(str.to_string()),
                _ => None
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn add_alias(ver: &str, alias: &str) -> Result<(), Box<dyn Error>> {
    let msg = "Adding R-".to_string() + alias + " alias";
    escalate(&msg)?;

    info!("Adding R-{} alias to R {}", alias, ver);

    let base = Path::new(R_ROOT);
    let target = base.join(ver).join("Resources/bin/R");
    let linkfile = Path::new("/usr/local/bin/").join("R-".to_string() + alias);

    // If it exists then we check that it points to the right place
    // Cannot use .exists(), because it follows symlinks
    let meta = std::fs::symlink_metadata(&linkfile);
    if meta.is_ok() {
        match std::fs::read_link(&linkfile) {
            Err(_) => bail!("{} is not a symlink, aborting", linkfile.display()),
            Ok(xtarget) => {
                if xtarget == target {
                    return Ok(())
                } else {
                    debug!("{} is wrong, updating", linkfile.display());
                    match std::fs::remove_file(&linkfile) {
                        Err(err) => {
                            bail!(
                                "Failed to delete {}, cannot update alias: {}",
                                linkfile.display(),
                                err.to_string()
                            );
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    // If we are still here, then we need to create the link
    debug!("Adding {} -> {}", linkfile.display(), target.display());
    match symlink(&target, &linkfile) {
        Err(err) => bail!(
            "Cannot create alias {}: {}",
            linkfile.display(),
            err.to_string()
        ),
        _ => {}
    };

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn add_alias(ver: String, alias: String) {
    // TODO
}

#[cfg(target_os = "linux")]
pub fn add_alias(ver: String, alias: String) {
    // TODO
}
