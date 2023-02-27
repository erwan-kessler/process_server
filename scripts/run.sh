#!/bin/bash
# this script will be run as this user
user="root"
echo "Will run script as $user, currently I'm $USER in $(pwd)"
# run the rest of the script as the user
tail -n +$((LINENO + 2)) "$0" | exec sudo -u $user /bin/bash
exit $? # exit for our runner

# continue as the user
echo "$USER is running the script"

#make sure to source his environment variables
source "$HOME/.bashrc"
# in case it is not defined use this line
# source "$HOME/.cargo/env"

if ! /bin/command -v rustc >/dev/null; then
	echo "Rust not found in user : $USER, go to https://rustup.rs"
	exit 1
fi

if ! /bin/command -v cargo >/dev/null; then
	echo "Cargo not found in user : $USER, either build it from sources or re install it with rustup"
	exit 1
fi

cargo run --release
