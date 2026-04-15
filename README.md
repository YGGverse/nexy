# Nexy - Multi-network server for the [Nex protocol](https://nightfall.city/nex/info/specification.txt)

![Linux](https://github.com/yggverse/nexy/actions/workflows/linux.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/yggverse/nexy/status.svg)](https://deps.rs/repo/github/yggverse/nexy)
[![crates.io](https://img.shields.io/crates/v/nexy)](https://crates.io/crates/nexy)

![Nexy UI](https://github.com/user-attachments/assets/3df0f044-0e13-4433-9e2d-77b6b725a884)

## Features

> [!TIP]
> See the [Options](#options) section for a complete list of other features!

* [x] Run IPv4/IPv6 server accessible to Internet, [Yggdrasil](https://yggdrasil-network.github.io/), [Mycelium](https://github.com/threefoldtech/mycelium), and other networks simultaneously, as many as desired;
* [x] Customizable templates for the directory index locations;
* [x] Build-in daily requests counter for the current session with template macro support;
* [x] Supports the [CLF](https://en.wikipedia.org/wiki/Common_Log_Format) access log, which is compatible with analytics tools such as [GoAccess](https://goaccess.io/), [GoatCounter](https://www.goatcounter.com/) or just [htcount](https://github.com/yggverse/htcount);
* [x] Custom templates for various server response types;
* [x] UTF-8 auto-slugs for directory index

## Install

1. `git clone https://github.com/yggverse/nexy.git && cd nexy`
2. `cargo build --release`
3. `sudo install target/release/nexy /usr/local/bin/nexy`

## Usage

> [!TIP]
> For more examples, visit the project [Wiki](https://github.com/YGGverse/nexy/wiki)

``` bash
RUST_LOG=TRACE nexy -p /path/to/public_dir
```
* by default, server starts on localhost; change it with the `--bind` option(s)

### Options

``` bash
nexy --help
```

## Live

* `nex://[202:68d0:f0d5:b88d:1d1a:555e:2f6b:3148]/` - [Yggdrasil](https://yggdrasil-network.github.io)
* `nex://[505:6847:c778:61a1:5c6d:e802:d291:8191]/` - [Mycelium](https://github.com/threefoldtech/mycelium)
* `nex://sl5ddrkufwd37xbbf4bj7542qljtnwe6pzd54epqg6zfytkj7q5a.b32.i2p/`

## See also

* [Yoda](https://github.com/YGGverse/Yoda) - Client for the Gemini & Nex protocols, written in Rust
* [snac2nex](https://crates.io/crates/snac2nex) - Export [Snac](https://codeberg.org/grunfink/snac2) profiles to the Nex blog format