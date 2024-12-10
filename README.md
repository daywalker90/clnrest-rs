[![latest release on CLN v24.11](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.11.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.11.yml) [![latest release on CLN v24.08.2](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.08.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.08.yml) [![latest release on CLN v24.05](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.05.yml)

[![main on CLN v24.11](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.11.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.11.yml) [![main on CLN v24.08.2](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.08.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.08.yml) [![main on CLN v24.05](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.05.yml)

# clnrest-rs
A rust rewrite of [clnrest](https://github.com/ElementsProject/lightning/tree/master/plugins/clnrest)

* [Installation](#installation)
* [Building](#building)
* [Thanks](#thanks)

# Installation
There is two ways you can do this, either edit the config or switch out the plugin file:

- Either replace `/usr/local/libexec/c-lightning/plugins/clnrest/clnrest` (can be different path depending on your OS) with the binary from this repo
- Or edit the config:

```
disable-plugin=clnrest
plugin=/path/to/clnrest-rs
```

Release binaries for
* x86_64-linux
* armv7-linux (Raspberry Pi 32bit)
* aarch64-linux (Raspberry Pi 64bit)

can be found on the [release](https://github.com/daywalker90/clnrest-rs/releases) page. If you are unsure about your architecture you can run ``uname -m``.

They require ``glibc>=2.31``, which you can check with ``ldd --version``.

# Building
You can build the plugin yourself instead of using the release binaries.
First clone the repo:

```
git clone https://github.com/daywalker90/clnrest-rs.git
```

Install a recent rust version ([rustup](https://rustup.rs/) is recommended) and in the ``clnrest-rs`` folder run:

```
cargo build --release
```

After that the binary will be here: ``target/release/clnrest-rs``

Note: Release binaries are built using ``cross`` and the ``optimized`` profile.

# Options

These work the same as with the python implementation:

### Required options
- ``clnrest-port``: REST server port to listen on

### Optional options
- ``clnrest-certs``: Path for certificates (for https), if you have your own certificates, otherwise clnrest will generate self-signed certs in your ``lightning-dir``
- ``clnrest-protocol``: REST server protocol (``https`` or ``http``), defaults to ``https``
- ``clnrest-host``: REST server host, defaults to ``127.0.0.1``
- ``clnrest-cors-origins``:Cross origin resource sharing origins, defaults to ``*``
- ``clnrest-csp``: Content security policy (CSP) for the server, defaults to ``default-src 'self'; font-src 'self'; img-src 'self' data:; frame-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline';``
- ``clnrest-swagger-root``: Root path for Swagger UI, defaults to ``/``


# Thanks
Thank you to the original author of clnrest [Shahana](https://github.com/ShahanaFarooqui)
