[![latest release on CLN v24.05](https://github.com/daywalker90/clnrest/actions/workflows/latest_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/latest_v24.05.yml) [![latest release on CLN v24.02.2](https://github.com/daywalker90/clnrest/actions/workflows/latest_v24.02.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/latest_v24.02.yml) [![latest release on CLN v23.11.2](https://github.com/daywalker90/clnrest/actions/workflows/latest_v23.11.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/latest_v23.11.yml)

[![main on CLN v24.05](https://github.com/daywalker90/clnrest/actions/workflows/main_v24.05.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/main_v24.05.yml) [![main on CLN v24.02.2](https://github.com/daywalker90/clnrest/actions/workflows/main_v24.02.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/main_v24.02.yml) [![main on CLN v23.11.2](https://github.com/daywalker90/clnrest/actions/workflows/main_v23.11.yml/badge.svg?branch=main)](https://github.com/daywalker90/clnrest/actions/workflows/main_v23.11.yml)

# clnrest
A rust rewrite of [clnrest](https://github.com/ElementsProject/lightning/tree/master/plugins/clnrest)

* [Installation](#installation)
* [Building](#building)
* [Thanks](#thanks)

# Installation
Replace `/usr/local/libexec/c-lightning/plugins/clnrest/clnrest` with the binary from this repo

Release binaries for
* x86_64-linux
* armv7-linux (Raspberry Pi 32bit)
* aarch64-linux (Raspberry Pi 64bit)

can be found on the [release](https://github.com/daywalker90/clnrest/releases) page. If you are unsure about your architecture you can run ``uname -m``.

They require ``glibc>=2.31``, which you can check with ``ldd --version``.

# Building
You can build the plugin yourself instead of using the release binaries.
First clone the repo:

```
git clone https://github.com/daywalker90/clnrest.git
```

Install a recent rust version ([rustup](https://rustup.rs/) is recommended) and in the ``clnrest`` folder run:

```
cargo build --release
```

After that the binary will be here: ``target/release/clnrest``

Note: Release binaries are built using ``cross`` and the ``optimized`` profile.


# Thanks
Thank you to the original author of clnrest [Shahana](https://github.com/ShahanaFarooqui)
