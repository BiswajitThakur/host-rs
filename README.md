# Host-RS

[![Crates.io](https://img.shields.io/crates/v/host-rs.svg)](https://crates.io/crates/host-rs)
[![downloads](https://img.shields.io/crates/d/host-rs.svg)](https://crates.io/crates/host-rs)

A CLI tool that manage website blocking, unblocking, redirection.

## Features

- [x] **Block Website:** Add websites to blocklist to prevent access.
- [x] **Unblock Website:** Remove websites from the blocklist to allow access.
- [x] **Redirect Website:** Redirect traffic from one website to another.
- [x] **Add Host Sources:** Add host source (A url that contains a list of hostnames).
- [x] **Update Host Sources:** Update host sources.
- [x] **Import hosts:** Import hostnames stored in a file and block or unblock them.
- [x] **Export hosts:** Export the list of currently blocked or unblocked hosts.

## Installation

Ensure you have Rust installed (If you dont want to install Rust, install from precompiled binary).

## Linux & macOS

```bash
git clone https://github.com/BiswajitThakur/host-rs.git
cd host-rs/
./install.sh
```

## Other

```
cargo install host-rs
```

## **Usage**

- **Help**

```
> host-rs --help
A CLI tool that manage website blocking, unblocking, redirection.

Usage: host-rs [OPTIONS] [COMMAND]

Commands:
  insert  Add host or url to allow, block, redirect, sources list
  remove  Remove allow, block, redirect host and host sources
  print   Print allow, block, redirect, etc host and host sources
  update  Update sources or self
  help    Print this message or the help of the given subcommand(s)

Options:
      --allow <ALLOW>...        Add host to allow list & removed from block list
      --block <BLOCK>...        Add to block list & remove from allow list
      --redirect <REDIRECT>...  Add to redirect list & remove from allow and block
      --restore-etc-hosts       restore /etc/hosts file
      --clear-all-data          Clear all data
      --export <EXPORT>         Expoer user data (you can import it later)
      --import <IMPORT>         Import data
  -h, --help                    Print help
  -V, --version                 Print version
```

- **Block Website**

```bash
host-rs --block <url or hostname1> <url or hostname2> ...
```

- **Remove Website from Blocklist**

```bash
host-rs rm --block <url or hostname1> <url or hostname2> ...
# or
# host-rs remove --block <url or hostname1> <url or hostname2> ...
```

- **Allow Website**

use this when, you added hostsources and if it block some website and you don't want to block it.

```bash
host-rs --allow <url or hostname1> <url or hostname2> ...
```

- **Remove Website from Allow List**

```bash
host-rs rm --allow <url or hostname1> <url or hostname2> ...
```

- **Redirection**

In the following example, `<url or hostname2>` redirected to `<url or hostname1>` and `<url or hostname4>` redirected to `<url or hostname3>`

```bash
host-rs --redirect <url or hostname1> <url or hostname2> <url or hostname3> <url or hostname4> ...
```

- **Remove from Redirect**

```bash
host-rs rm --redirect <url or hostname2> <url or hostname4> ...
```

- **Add Host Sources**

You will get verious sources from [this](https://github.com/StevenBlack/hosts) repo.

```bash
host-rs insert --sources <url of sources>
```

- **Update Host Sources**

```bash
host-rs update --sources
```

- **Remove Host Sources**

```bash
host-rs remove --sources <url of sources>
```

- **Uninstall**

```bash
host-rs remove --self
```

## **Contributing**

Contributions are welcome! If you find any bugs, want to request a new feature or improve the code feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
