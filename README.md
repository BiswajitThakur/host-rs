# !!! Main branch is host-rs 1.0.0, under developing. !!!

For current main released version is in the [host-rs v0.1.2](https://github.com/BiswajitThakur/host-rs/tree/Host-RS_v0.1.2)

# Hosr-RS

This is a CLI tool written in Rust to manage website blocking, unblocking, redirection.

## Features

- [x] **Block Website:** Add websites to blocklist to prevent access.
- [x] **Unblock Website:** Remove websites from the blocklist to allow access.
- [x] **Redirect Website:** Redirect traffic from one website to another.
- [x] **Add Host Sources:** Add host source (A url that contains a list of hostnames).
- [x] **Update Host Sources:** Update host sources.
- [x] **Import hosts:** Import hostnames stored in a file and block or unblock them.
- [x] **Export hosts:** Export the list of currently blocked or unblocked hosts.
- [x] **Small Binary Size:** Small binary size, less then 3 MB.

## Installation

Ensure you have Rust installed (If you dont want to install Rust, install from precompiled binary).

```bash
git clone https://github.com/BiswajitThakur/host-rs.git
cd host-rs/
./scripts/build
sudo ./scripts/install
```

or

```
cargo install host-rs
```
