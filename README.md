# jack-latency-updater

A simple GTK4 application written in Rust to configure PipeWire quantum and sample rate.

![Jack Latency Updater](https://raw.githubusercontent.com/rolodoom/jack-latency-updater/master/screenshot.png)

## Status

[![GitHub license](https://img.shields.io/badge/license-GPL--3.0-blue)](https://raw.githubusercontent.com/rolodoom/jack-latency-updater/master/LICENSE)

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
git clone https://github.com/rolodoom/jack-latency-updater.git
cd jack-latency-updater
```

Build the release binary:

```bash
cargo build --release
```

The executable will be available at:

```text
target/release/jack-latency-updater
```

## Run

```bash
cargo run --release
```

## Bugs and Issues

Have a bug or an issue with this template? [Open a new issue](https://github.com/rolodoom/jack-latency-updater/issues) here on GitHub.

## License

This code is released under the [GPL-3.0](https://github.com/rolodoom/jack-latency-updater/blob/master/LICENSE) license, which means you have the four freedoms to run, study, share, and modify the software. Any derivative work must be distributed under the same or equivalent license terms.
