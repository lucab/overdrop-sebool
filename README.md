# overdrop-sebool

[![Build Status](https://travis-ci.org/overdrop/overdrop-sebool.svg?branch=master)](https://travis-ci.org/overdrop/overdrop-sebool)

A small Rust binary to manage SELinux booleans at runtime.

It allows to tweak SELinux boolean values and persist changes across
reboots, via TOML configuration files. It is targeted toward
early-boot configuration of an immutable OS, and aims at [decoupling
configuration concerns](https://github.com/projectatomic/rpm-ostree/issues/27)
regarding vendor-defaults, user-configuration and internal/runtime state.

This project follows the systemd-style approach of **over**laying
**drop**in snippets from multiple hierarchies (i.e. `/lib`, `/run`,
and `/etc`), thus the name.

It does not have any additional non-Rust runtime dependency, that is
it doesn't depend on having a `libselinux.so` on the target host.

# Demo

This binary can be directly used as a systemd service to setup
SELinux booleans at early-boot. A live-action demo of that is in
the following asciinema recording:

[![asciicast](https://asciinema.org/a/200268.png)](https://asciinema.org/a/200268)

# Disclaimer

This project is an early proof-of-concept, and it may expose some raw
edges or unexpected behavior.
