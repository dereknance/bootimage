use super::error::DiskImageError;
use std::fs::OpenOptions;
use std::io::ErrorKind::AlreadyExists;
use std::io::Write;
use std::{path::Path, process::Command};

pub fn create_iso_image(
    output_bin_path: &Path,
    isodir: &Path,
    bin_path: &Path,
    bin_name: &str,
) -> Result<(), DiskImageError> {
    match std::fs::create_dir(isodir) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == AlreadyExists {
                Ok(())
            } else {
                Err(DiskImageError::Io {
                    message: "failed to create isodir",
                    error: e,
                })
            }
        }
    }?;

    let grub_dir = isodir.join("boot/grub");
    match std::fs::create_dir_all(&grub_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == AlreadyExists {
                Ok(())
            } else {
                Err(DiskImageError::Io {
                    message: "failed to create boot/grub",
                    error: e,
                })
            }
        }
    }?;

    let mut grubcfg = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&grub_dir.join("grub.cfg"))
        .map_err(|err| DiskImageError::Io {
            message: "failed to open grub.cfg",
            error: err,
        })?;

    grubcfg
        .write(
            format!(
                r#"
        set timeout=0
        set default=0

        menuentry "{}" {{
            multiboot2 /boot/kernel.elf
            boot
        }}
        "#,
                bin_name
            )
            .as_bytes(),
        )
        .map_err(|err| DiskImageError::Io {
            message: "failed to write grub.cfg",
            error: err,
        })?;

    std::fs::copy(bin_path, isodir.join("boot/kernel.elf")).map_err(|err| {
        DiskImageError::Io {
            message: "failed to create kernel.elf",
            error: err,
        }
    })?;

    let mut cmd = Command::new("grub-mkrescue");
    cmd.arg("-o").arg(output_bin_path);
    cmd.arg(isodir);

    let output = cmd.output().map_err(|err| DiskImageError::Io {
        message: "failed to execute grub-mkrescue command",
        error: err,
    })?;
    if !output.status.success() {
        return Err(DiskImageError::MkResuceFailed {
            stderr: output.stderr,
        });
    }

    Ok(())
}
