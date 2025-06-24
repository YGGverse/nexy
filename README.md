# Nexy - Multi-network server for the [Nex protocol](https://nightfall.city/nex/info/specification.txt)

![Build](https://github.com/yggverse/nexy/actions/workflows/build.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/yggverse/server/status.svg)](https://deps.rs/repo/github/yggverse/nexy)
[![crates.io](https://img.shields.io/crates/v/nexy)](https://crates.io/crates/nexy)

Run server accessible to Internet IPv4/IPv6, [Yggdrasil](https://yggdrasil-network.github.io/), [Mycelium](https://github.com/threefoldtech/mycelium), and other networks simultaneously, as many as desired. Optimized for streaming large files (in chunks) without memory overload on reading.

## Install

1. `git clone https://github.com/yggverse/nexy.git && cd nexy`
2. `cargo build --release`
3. `sudo install target/release/nexy /usr/local/bin/nexy`

## Usage

``` bash
nexy -p /path/to/public_dir
```
* by default, server starts on localhost; change it with the `--bind` option(s)

### Options

``` bash
-b, --bind <BIND>
        Bind server(s) `host:port` to listen incoming connections

        * use `[host]:port` notation for IPv6

        [default: 127.0.0.1:1900 [::1]:1900]

-d, --debug <DEBUG>
        Debug level

        * `e` - error * `i` - info

        [default: ei]

-p, --public <PUBLIC>
        Absolute path to the public files directory

--template-access-denied <TEMPLATE_ACCESS_DENIED>
        Absolute path to the `Access denied` template file

--template-internal-server-error <TEMPLATE_INTERNAL_SERVER_ERROR>
        Absolute path to the `Internal server error` template file

--template-not-found <TEMPLATE_NOT_FOUND>
        Absolute path to the `Not found` template file

--template-welcome <TEMPLATE_WELCOME>
        Absolute path to the `Welcome` template file. Unlike `template_index`, this applies only to the `public` location

        **Patterns** * `{list}` - entries list for the `public` directory

--template-index <TEMPLATE_INDEX>
        Absolute path to the `Index` template file for each directory

        **Patterns** * `{list}` - entries list for the current directory

-r, --read-chunk <READ_CHUNK>
        Optimize memory usage on reading large files or stream

        [default: 1024]

-h, --help
        Print help (see a summary with '-h')

-V, --version
        Print version
```