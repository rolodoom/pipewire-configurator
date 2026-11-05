# pipewire-configurator

A simple GTK4 application written in Rust to configure PipeWire quantum and sample rate.

![Jack Latency Updater](https://raw.githubusercontent.com/rolodoom/pipewire-configurator/master/screenshot.png)

## Status

[![GitHub license](https://img.shields.io/badge/license-GPL--3.0-blue)](https://raw.githubusercontent.com/rolodoom/pipewire-configurator/master/LICENSE)

## Features

- Configure PipeWire quantum using a dropdown selector.
- Configure PipeWire sample rate using a dropdown selector.
- Read the currently applied quantum and sample rate when the application starts.
- Apply settings directly through `pw-metadata`.

## Dependencies

On Debian-based distributions:

```bash
sudo apt install pkg-config libgtk-4-dev
```

You also need the Rust toolchain and Cargo.

## Build

Clone the repository:

```bash
git clone https://github.com/user/pipewire-configurator.git
cd pipewire-configurator
```

Build the release binary:

```bash
cargo build --release
```

The executable will be available at:

```text
target/release/pipewire-configurator
```

## Run

```bash
cargo run --release
```

## Build Debian package (.deb)

Install `cargo-deb` if you haven't already:

```bash
cargo install cargo-deb
```

Build the release binary and generate the Debian package:

```bash
cargo build --release
cargo deb
```

The generated `.deb` package will be available in:

```text
target/debian/
```

Install the package locally:

```bash
sudo apt install ./target/debian/pipewire-configurator_*.deb
```

The package automatically declares its shared library dependencies using `cargo-deb`'s `$auto` mechanism.

## Bugs and Issues

Have a bug or an issue with this template? [Open a new issue](https://github.com/rolodoom/pipewire-configurator/issues) here on GitHub.

## License

This code is released under the [GPL-3.0](https://github.com/rolodoom/pipewire-configurator/blob/master/LICENSE) license, which means you have the four freedoms to run, study, share, and modify the software. Any derivative work must be distributed under the same or equivalent license terms.
