#!/bin/bash

if [ -x "$(command -v cargo)" ]; then
    echo "SUCCESS: Rust is installed"
else
    echo "ERROR: Rust does not seem to be installed."
    read -p "Do you want to install Rust (y/n): " rs
    if [ $rs = "y" ]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    fi
fi
