<p align="center">
  <h1 align="center">cfdydns</h1>
</p>

**Cloudflare dynamic DNS client**

[![crates-badge](https://img.shields.io/crates/v/cfdydns.svg)](https://crates.io/crates/cfdydns)

A simplistic dynamic DNS client that only works with Cloudflare, for only one domain. It's easily configurable via environment variables, making it an ideal choice to use in containers.

## Installation

The recommended way of using `cfdydns` is through containers. Built containers are available in:

- [GitHub Container Registry](https://github.com/xJonathanLEI/cfdydns/pkgs/container/cfdydns): `ghcr.io/xjonathanlei/cfdydns:latest`
- [Docker Hub](https://hub.docker.com/r/xjonathanlei/cfdydns): `xjonathanlei/cfdydns:latest`

You may also install the binary directly. With the Rust toolchain installed:

```console
cargo install --locked --path .
```

## Getting started

`cfdydns` is configurable via command line options and environment variables. Running `cfdydns --help` reveals the options:

| Option        | Env var             | Optional | Description                                                              |
| ------------- | ------------------- | -------- | ------------------------------------------------------------------------ |
| `--fqdn`      | `CFDYDNS_FQDN`      | No       | Fully-qualified domain name to set A record on                           |
| `--zone`      | `CFDYDNS_ZONE`      | No       | Zone name of the FQDN (e.g. `example.com`)                               |
| `--api-token` | `CFDYDNS_API_TOKEN` | No       | Cloudflare API token with the `DNS: Edit` permission for the target zone |
| `--interval`  | `CFDYDNS_INTERVAL`  | Yes      | Number of seconds to wait between each check                             |

> [!TIP]
>
> `cfdydns` does not create new records for you. The target `A` record for the FQDN must already exist.

## Why not [ddclient](https://github.com/ddclient/ddclient)?

`ddclient` is hard to configure and barely maintained. Who cares about all the DNS provider integration when you're only using Cloudflare anyways.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
