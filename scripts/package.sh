#!/usr/bin/env bash
set -ex

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
parent_dir="$(dirname "${dir}")"

echo --- Creating sbf-sdk tarball

rm -rf sbf-sdk.tar.bz2 sbf-sdk/
mkdir sbf-sdk/
cp LICENSE sbf-sdk/

(
  "${dir}"/crate-version.sh "${parent_dir}"/Cargo.toml
  git rev-parse HEAD
) > sbf-sdk/version.txt

cp -a sbf/* sbf-sdk/

tar jvcf sbf-sdk.tar.bz2 sbf-sdk/
rm -rf sbf-sdk
