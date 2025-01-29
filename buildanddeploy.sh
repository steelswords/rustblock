#!/usr/bin/env bash
# File:        buildanddeploy.sh
# Author:      Tristan Andrus
# Description: Builds for my router and deploys it
################################################################################

set -o errexit   # Abort on nonzero exitstatus
set -o nounset   # Abort on unbound variable
set -o pipefail  # Don't hide errors within pipes

trap "echo 'An error occurred! Quitting mid-script!'" ERR

# Uncomment to debug
# set -x

################################################################################

cross build --target armv7-unknown-linux-musleabi --release

scp -O ./target/armv7-unknown-linux-musleabi/release/rustblock \
    ./websites.toml \
    ./blockprofiles.toml \
    router:/jffs
