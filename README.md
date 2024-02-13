# Host-RS

Block Website, Ads, Social, Porn, Fakenews, Gambling, Spam sites and website redirector.

> [!NOTE]
> Administrator privileges required to perform any block or unblock operation.

## Installation

### Install from source code on Linux
```bash
git clone https://github.com/BiswajitThakur/host-rs
cd host-rs
./setup.sh
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

```
# host-rs --block-web <hostname0> <hostname1> ...
```

or

```
# host-rs -b <hostname0> <hostname1> ...
```

or

```
# host-rs --bw <hostname0> <hostname1> ...
```

### Unblock web

```
# host-rs --unblock-web <hostname0> <hostname1> ...
```

or

```
# host-rs -u <hostname0> <hostname1> ...
```

or

```
# host-rs --uw <hostname0> <hostname1> ...
```

### Redirect websites

```
# host-rs --add-redirect <hostname0> <hostname1>
```

or

```
# host-rs --ar <hostname0> <hostname1>
```

In above example, &lt;hostname1&gt; redirected to &lt;hostname0&gt;

### Remove Redirect

```
# host-rs --rm-redirect <hostname0> <hostname1> ...
```

or 

```
# host-rs --rr <hostname0> <hostname1> ...
```

### Block ads

```
# host-rs --block-ads
```

or

```
# host-rs --ba
```

### Unblock ads

```
# host-rs --unblock-ads
```

or

```
# host-rs --ua
```

or

```
# host-rs --uba
```

### Block porn

```
# host-rs --block-porn
```

or

```
# host-rs --bp
```

### Unblock porn

```
# host-rs --unblock-porn
```

or

```
# host-rs --up
```

or

```
# host-rs --ubp
```

### Block fakenews

```
# host-rs --block-fakenews
```

or

```
# host-rs --bf
```

### Unblock fakenews

```
# host-rs --unblock-fakenews
```

or

```
# host-rs --uf
```

or

```
# host-rs --ubf
```

### Block social

```
# host-rs --block-social
```

or

```
# host-rs --bs
```

### Unblock social

```
# host-rs --unblock-social
```

or

```
# host-rs --us
```

or

```
# host-rs --ubs
```

### Block gambling

```
# host-rs --block-gambling
```

or

```
# host-rs --bg
```

### Unblock gambling

```
# host-rs --unblock-gambling
```

or

```
# host-rs --ug
```

or

```
# host-rs --ubg
```

### Update host sources

```
# host-rs --update-sources
```

### Update

This features is not implemented yet.

```
# host-rs --update
```

or

```
# host-rs --update-self
```

### Uninstall

```
# host-rs --remove-self
```

or

```
# host-rs --uninstall
```
