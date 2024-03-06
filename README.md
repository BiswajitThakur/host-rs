# Host-RS

Block Website, Ads, Social, Porn, Fakenews, Gambling, Spam sites and website redirector.

> [!NOTE]
> Administrator privileges required to perform any block or unblock operation.

> [!NOTE]
> This application does not block YouTube's ads.

## Installation

### Install from source code on Linux
```bash
git clone https://github.com/BiswajitThakur/host-rs
cd host-rs
make build
sudo make install
```
### Install from precompiled binary
1. [Click here to Download](https://github.com/BiswajitThakur/host-rs/releases) - download sutable binary for your system
1. `cd ~/Downloads/` - Go to the location of the binarry file
1. `chmod +x installer_host-rs_*.sh` - Add executable permission
1. `sudo ./installer_host-rs_*.sh` - Execute

## Features of Host-RS
- [x] Block websites
- [x] Unblock websites
- [x] Redirect website
- [x] Block Ads
- [x] Unblock ads
- [x] Block all porn sites
- [x] Unblock porn websites
- [x] Block fakenews websites
- [x] Unblock fakenews websites
- [x] Block all social websites
- [x] Unlock social websites
- [x] Block gambling websites
- [x] Unblock gambling websites

## Help
```
> host-rs --help
Usage: host-rs [OPTIONS]

Options:
  -b, --block-web <block-web>...                    Block websites.
  -u, --unblock-web <unblock-web>...                Unblock websites.
      --add-redirect <add-redirect> <add-redirect>  Redirect website
      --rm-redirect <rm-redirect>...                remove from redirect
      --block-ads                                   Block Ads.
      --unblock-ads                                 Unblock all ads.
      --block-porn                                  Block all porn sites.
      --unblock-porn                                Unblock porn websites.
      --block-fakenews                              Block fakenews websites.
      --unblock-fakenews                            Unblock fakenews websites.
      --block-social                                Block all social websites.
      --unblock-social                              Unlock social websites.
      --block-gambling                              Block gambling websites.
      --unblock-gambling                            Unblock gambling websites.
      --update-self                                 Update if available.
      --remove-self                                 Uninstall
      --update-sources                              Update host sources if available.
  -h, --help                                        Print help
  -V, --version                                     Print version

```

## Examples

### Block web

```bash
host-rs --block-web <hostname0> <hostname1> ...
```

or

```bash
host-rs -b <hostname0> <hostname1> ...
```

or

```bash
host-rs --bw <hostname0> <hostname1> ...
```

### Unblock web

```bash
host-rs --unblock-web <hostname0> <hostname1> ...
```

or

```bash
host-rs -u <hostname0> <hostname1> ...
```

or

```bash
host-rs --uw <hostname0> <hostname1> ...
```

### Redirect websites

```bash
host-rs --add-redirect <hostname0> <hostname1>
```

or

```bash
host-rs --ar <hostname0> <hostname1>
```

In above example, &lt;hostname1&gt; redirected to &lt;hostname0&gt;

### Remove Redirect

```bash
host-rs --rm-redirect <hostname0> <hostname1> ...
```

or 

```bash
host-rs --rr <hostname0> <hostname1> ...
```

### Block ads

```bash
host-rs --block-ads
```

or

```bash
host-rs --ba
```

### Unblock ads

```bash
host-rs --unblock-ads
```

or

```bash
host-rs --ua
```

or

```bash
host-rs --uba
```

### Block porn

```bash
host-rs --block-porn
```

or

```bash
host-rs --bp
```

### Unblock porn

```bash
host-rs --unblock-porn
```

or

```bash
host-rs --up
```

or

```bash
host-rs --ubp
```

### Block fakenews

```bash
host-rs --block-fakenews
```

or

```bash
host-rs --bf
```

### Unblock fakenews

```bash
host-rs --unblock-fakenews
```

or

```bash
host-rs --uf
```

or

```bash
host-rs --ubf
```

### Block social

```bash
host-rs --block-social
```

or

```bash
host-rs --bs
```

### Unblock social

```bash
host-rs --unblock-social
```

or

```bash
host-rs --us
```

or

```bash
host-rs --ubs
```

### Block gambling

```bash
host-rs --block-gambling
```

or

```bash
host-rs --bg
```

### Unblock gambling

```bash
host-rs --unblock-gambling
```

or

```bash
host-rs --ug
```

or

```bash
host-rs --ubg
```

### Update host sources

```bash
host-rs --update-sources
```

### Update

This features is not implemented yet.

```bash
host-rs --update
```

or

```bash
host-rs --update-self
```

### Uninstall

```bash
host-rs --remove-self
```

or

```bash
host-rs --uninstall
```
