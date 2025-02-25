#!/bin/env bash

bin="host-rs"

host_rs_path=$HOME/.cargo/bin/$bin
if [[ -f /usr/bin/$bin ]]; then
  rm /usr/bin/$bin;
fi
if [[ $? -ne 0 ]]; then
  echo "ERROR: faild to remove '$bin'";
  exit 1
fi

if [[ -f $host_rs_path ]]; then
  rm &host_rs_path
fi
if [[ $? -ne 0 ]]; then
  echo "ERROR: faild to remove '$host_rs_path'";
  exit 1
fi

echo ".....Uninstall Success....."
