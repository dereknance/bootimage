# grubimage

Creates a bootable disk image from a Rust OS kernel.

## Prerequisites

Before `grubimage` will successfully build and run images, the following packages must be installed (package names come from Debian-based systems):

* `grub-common` (for `grub-mkrescue`)
* `grub-pc-bin` (only required for EFI-based hosts)
* `xorriso`
* `qemu-system-x86`

## Installation

```
> cargo install grubimage
```

## Usage

The `grub-mkrescue` program must be on somewhere on the `$PATH`.

### Building

Now you can build the kernel project and create a bootable disk image from it by running:

```
cargo grubimage --target your_custom_target.json [other_args]
```

The command will invoke `cargo build`, forwarding all passed options. Then it will build the specified bootloader together with the kernel to create a bootable disk image.

### Build with Grub
You can set the `--grub` flag to enable grub compilation mode. It uses `grub-mkrescue` to generate a bootable iso with your kernel inside linked with the bootloader crate of your choosing.
The bootloader crate must support the multiboot2 specification.

### Running

To run your kernel in QEMU, you can set a `grubimage runner` as a custom runner in a `.cargo/config` file:

```toml
[target.'cfg(target_os = "none")']
runner = "grubimage runner"
```

Then you can run your kernel through:

```
cargo xrun --target your_custom_target.json [other_args] -- [qemu args]
```

All arguments after `--` are passed to QEMU. If you want to use a custom run command, see the _Configuration_ section below.

### Testing

The `grubimage` has built-in support for running unit and integration tests of your kernel. For this, you need to use the `custom_tests_framework` feature of Rust as described [here](https://os.phil-opp.com/testing/#custom-test-frameworks).

## Configuration

Configuration is done through a through a `[package.metadata.grubimage]` table in the `Cargo.toml` of your kernel. The following options are available:

```toml
[package.metadata.grubimage]
# The cargo subcommand that will be used for building the kernel.
#
# For building using the `cargo-xbuild` crate, set this to `xbuild`.
build-command = ["build"]
# The command invoked with the created grubimage (the "{}" will be replaced
# with the path to the bootable disk image)
# Applies to `grubimage run` and `grubimage runner`
run-command = ["qemu-system-i386", "-cdrom", "{}"]

# Additional arguments passed to the run command for non-test executables
# Applies to `grubimage run` and `grubimage runner`
run-args = []

# Additional arguments passed to the run command for test executables
# Applies to `grubimage runner`
test-args = []

# An exit code that should be considered as success for test executables
test-success-exit-code = {integer}

# The timeout for running a test through `grubimage test` or `grubimage runner` (in seconds)
test-timeout = 300

# Whether the `-no-reboot` flag should be passed to test executables
test-no-reboot = true
```

### Inner workings
The `grubimage` command first reads the `CARGO_MANIFEST_DIR` environment variable to find out where the `Cargo.toml` of the current project is located.
Then it parses the `Cargo.toml` file to read grubimage specific configuration data out of it. It then proceeds to build the current cargo project.
Afterwards it creates following folder structure in the same directory of your kernel executable:
```
target/<target>/debug/isofiles
└── boot
    ├── grub
    │   └── grub.cfg
    └── kernel.elf
```
It then executes `grub-mkrescue -o <output_bin_path>.iso <target_path>/isofiles` to create a bootable grub iso image.
This mode is not compatible with the [bootloader](https://github.com/rust-osdev/bootloader) crate from rust-osdev.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
