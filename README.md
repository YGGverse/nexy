# Nexy - Multi-network server for the [Nex protocol](https://nightfall.city/nex/info/specification.txt)

![Build](https://github.com/yggverse/nexy/actions/workflows/build.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/yggverse/nexy/status.svg)](https://deps.rs/repo/github/yggverse/nexy)
[![crates.io](https://img.shields.io/crates/v/nexy)](https://crates.io/crates/nexy)

## Features

* Run IPv4/IPv6 server accessible to Internet, [Yggdrasil](https://yggdrasil-network.github.io/), [Mycelium](https://github.com/threefoldtech/mycelium), and other networks simultaneously, as many as desired;
* Optimized for streaming large files (in chunks) without memory overload on buffering the data;
* Supports the [CLF](https://en.wikipedia.org/wiki/Common_Log_Format) access log, which is compatible with analytics tools such as [GoAccess](https://goaccess.io/), [GoatCounter](https://www.goatcounter.com/) or just [htcount](https://github.com/yggverse/htcount);
* Custom templates for various server response types;
* See the [Options](#options) section for a complete list of other features.

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
-a, --access-log <ACCESS_LOG>
        Absolute path to the access log file

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

        * this template file can be in binary format (e.g. image)

--template-internal-server-error <TEMPLATE_INTERNAL_SERVER_ERROR>
        Absolute path to the `Internal server error` template file

        * this template file can be in binary format (e.g. image)

--template-not-found <TEMPLATE_NOT_FOUND>
        Absolute path to the `Not found` template file

        * this template file can be in binary format (e.g. image)

--template-welcome <TEMPLATE_WELCOME>
        Absolute path to the `Welcome` template file. Unlike `template-index`, this applies only to the `public` location

        * this template file expects pattern and cannot be in binary format

        **Patterns** * `{list}` - entries list for the `public` directory

--template-index <TEMPLATE_INDEX>
        Absolute path to the `Index` template file for each directory

        * this template file expects pattern and cannot be in binary format

        **Patterns** * `{list}` - entries list for the current directory

--list-dir-count
        Show files count in dir (as the alternative text for navigation links)

--list-dir-accessed
        Show directory accessed time

--list-dir-created
        Show directory created time

--list-dir-modified
        Show directory modified time

--list-dir-reverse
        Sort directories in list DESC (ASC by default)

--list-file-size
        Show file size in list (as the alternative text for navigation links)

--list-file-accessed
        Show file accessed time

--list-file-created
        Show file created time

--list-file-modified
        Show file modified time

--list-file-reverse
        Sort files in list DESC (ASC by default)

--list-time-format <LIST_TIME_FORMAT>
        Time format for listing items

        * use escape notation for `%` e.g. `"%%Y-%%m-%%d %%H:%%M:%%S"`

        [default: "%Y-%m-%d %H:%M:%S"]

-r, --read-chunk <READ_CHUNK>
        Optimize memory usage on reading large files or stream

        [default: 1024]

-h, --help
        Print help (see a summary with '-h')

-V, --version
        Print version
```