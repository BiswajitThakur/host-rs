# Host-RS

Block Website, Ads, Social, Porn, Fakenews, Gambling, Spam sites and website redirector.

> [!NOTE]
> Administrator privileges required to perform any block or unblock operation.

> [!WARNING]
> Work in progress

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