#!/bin/env bash

prog="host-rs"

function print_success {
    green='\033[0;32m'
    reset='\033[0m'
    echo -e "${green}$@${reset}"
}

function print_faild {
    red='\033[0;31m'
    reset='\033[0m'
    echo -e "${red}$@${reset}"
}

function check_root {
    if [[ "$(id -u)" -ne 0 ]]; then
        print_faild "This operation must be run as root. Please use sudo";
        exit 1
    fi
}

function get_version {
    echo $($1 --version | awk "/$prog/ {print \$2}")
}


function get_install_dir {
    if [ -d /data/data/com.termux/files/usr/bin ]; then
        echo '/data/data/com.termux/files/usr/bin'
    elif [ -d /usr/bin ]; then
        echo '/usr/bin'
    fi
}

function compare_version {
    prog="host-rs"
    if [ -f "$(get_install_dir)/$prog" ]; then
        IFS='.' read -ra old_version <<< $(get_version "$(get_install_dir)/$prog")
        IFS='.' read -ra new_version <<< $(get_version ./target/release/$prog)
        if (( ${new_version[0]} == ${old_version[0]} )) &&
           (( ${new_version[1]} == ${old_version[1]} )) &&
           (( ${new_version[2]} == ${old_version[2]} )); then
            echo "equal"
        elif (( ${new_version[0]} >= ${old_version[0]} )) &&
           (( ${new_version[1]} >= ${old_version[1]} )) &&
           (( ${new_version[2]} >= ${old_version[2]} )); then
            echo "higher"
        else
            echo "old"
        fi
    else
        echo "new"
    fi
}

function install_update {
    if [ -d $1 ]; then
        if [ ! -f target/release/$prog ]; then
            cargo build --release
            if [[ $? -ne 0 ]]; then
                print_faild "Faild to build"
                exit 1
            fi
        fi
        if [ -f "$(get_install_dir)/$prog" ]; then
            rm -f "$(get_install_dir)/$prog"
            if [[ $? -ne 0 ]]; then
                exit 1
            fi
        fi
        echo "+--------------------------------------+"
        echo "| Binary: $prog                      |"
        echo "| Version: $(get_version target/release/$prog)                       |"
        echo "| Size: $( du -h target/release/$prog | awk '{print $1}')                           |"
        echo "+--------------------------------------+"
        cp target/release/$prog "$(get_install_dir)/"
        if [[ $? -eq 0 ]]; then
            echo -e " Installation status: \033[0;32mSUCCESS\033[0m"
            exit 0
        else
            echo -e " Installation status: \033[0;31mFAILD\033[0m"
            exit 1
        fi
    else
        print_faild "$1: Not found."
        exit 1
    fi
}

function install {
    check_root;
    comp=$(compare_version)
    if [ $comp == "old" ]; then
        echo "+---------------------------------------+"
        echo "| You are already using higher version. |"
        echo "+---------------------------------------+"
        echo "| Installed version: $(get_version $(get_install_dir)/$prog)              |"
        echo "|              size: $(du -h $(get_install_dir)/$prog | awk '{print $1'})               |"
        echo "+---------------------------------------+"
        echo ""
        echo "+---------------------------------------+"
        echo "| Do you want to install lower version  |"
        echo "|          version: $(get_version ./target/release/$prog)               |"
        echo "|             size: $( du -h ./target/release/$prog | awk '{print $1}')                |"
        read -p "+--- y/n: -------------------> " inp
        if [[ $inp == "y" ]]; then
            install_update;
        fi
    elif [ $comp == "equal" ]; then
        echo "+---------------------------------------+"
        echo "| You have already installed.           |"
        echo "+---------------------------------------+"
        echo "| Installed version: $(get_version $(get_install_dir)/$prog)              |"
        echo "|              size: $(du -h $(get_install_dir)/$prog | awk '{print $1'})               |"
        echo "+---------------------------------------+"
        echo ""
        echo " Do you want to install same version again"
        read -p "--- y/n: -------------------> " inp
        if [[ $inp == "y" ]]; then
            install_update;
        fi
    else
        install_update;
    fi
}

