#!/bin/env bash

function open_url {
  if [[ -x "$(command -v google-chrome)" ]];then
    google-chrome $1;
  elif [[ -x "$(command -v google-chrome-stable)" ]]; then
    google-chrome-stable $1;
  elif [[ -x "$(command -v firefox)" ]]; then
    firefox $1;
  else
    echo "Please open: $1"
  fi
}

bin="host-rs"

cargo install $bin;

if [[ $? -ne 0 ]]; then
  open_url "https://www.rust-lang.org/tools/install"; 
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

echo "Install success....."
