#!/bin/bash
#
# Sets up the data needed to run the integration tests in this directory.
#
# First, the minecraft-assets repo is fetched by doing:
#   $ git submodule init
#   $ git submodule update
#
# Then each of the versions that are tested are checked out as a separate
# git worktree in `assets-${VERSION}/`.

set -e

SCRIPT_PATH=$(realpath "$0")

TESTS_DIR=$(dirname "${SCRIPT_PATH}")

ASSETS_DIR="${TESTS_DIR}/minecraft-assets"

checkout_assets() {
    VERSION=$1
    VERSION_DIR="${TESTS_DIR}/assets-${VERSION}"

    if [ ! -e "${VERSION_DIR}/.git" ]; then
        echo "============ Checking out assets version ${VERSION} ============="

        git -C "${ASSETS_DIR}" worktree add "${VERSION_DIR}" "${VERSION}"
    else
        echo "========= Already checked out assets version ${VERSION} ========="
    fi
}

if [ ! -d "${ASSETS_DIR}/.git" ]; then
    echo "=============== Initializing submodules ==============="
    git submodule init

    echo "================= Updating submodules ================="
    git submodule update
fi

checkout_assets "1.8"
checkout_assets "1.9"
checkout_assets "1.11"
checkout_assets "1.12"
checkout_assets "1.13"
checkout_assets "1.14"
checkout_assets "1.15"
checkout_assets "1.16.2"
checkout_assets "1.17"
checkout_assets "1.18"
