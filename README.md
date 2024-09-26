[![latest release on CLN v24.08.1](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.08.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.08.yml) [![latest release on CLN v24.05](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.05.yml) [![latest release on CLN v24.02.2](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.02.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/latest_v24.02.yml)

[![main on CLN v24.08.1](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.08.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.08.yml) [![main on CLN v24.05](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.05.yml) [![main on CLN v24.02.2](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.02.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest-rs/actions/workflows/main_v24.02.yml)

# clnrest-rs
A rust rewrite of [clnrest](https://github.com/ElementsProject/lightning/tree/master/plugins/clnrest)

* [Installation](#installation)
* [Building](#building)
* [Thanks](#thanks)

# Installation
There is two ways you can do this, either edit the config or switch out the plugin file:

### CLN v24.05 and later
- Either replace `/usr/local/libexec/c-lightning/plugins/clnrest/clnrest` (can be different path depending on your OS) with the binary from this repo
- Or edit the config:

```
disable-plugin=clnrest
plugin=/path/to/clnrest-rs
```

### CLN v24.02.2 and older
- Either replace `/usr/local/libexec/c-lightning/plugins/clnrest/clnrest.py` (can be different path depending on your OS)  with the binary from this repo.
- Or edit the config:

```
disable-plugin=clnrest.py
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


# Thanks
Thank you to the original author of clnrest [Shahana](https://github.com/ShahanaFarooqui)
