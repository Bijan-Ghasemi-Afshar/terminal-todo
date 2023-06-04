#!/bin/bash

install_dir="/usr/local"

sudo mkdir -p "$install_dir"

cp target/release/terminal-todo $install_dir

cd "$install_dir" || exit

sudo chmod +x terminal-todo

echo "export PATH=\"$install_dir:\$PATH\"" >> ~/.bashrc

echo "Installation completed"
