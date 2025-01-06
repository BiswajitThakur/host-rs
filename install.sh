#!/bin/env bash

bin="host-rs"

cargo install $bin;

if [[ $? -ne 0 ]]; then
  open "https://www.rust-lang.org/tools/install"; 
  exit 1;
fi

host_rs_path=$HOME/.cargo/bin/$bin

if [[ -x $host_rs_path ]]; then
  if [[ ! -e /usr/bin/$bin ]]; then
    sudo ln -s $host_rs_path /usr/bin/$bin;
  fi
  if [[ $? -ne 0 ]]; then
    echo "ERROR: faild to create symbolic link";
    exit 1
  fi
fi

echo ".....Install Success....."
