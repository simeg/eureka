#!/usr/bin/env bash

set -e

readonly KCOV_BUILD="kcov-build"
readonly KCOV_OUT="target/cov"
readonly KCOV="./$KCOV_BUILD/usr/local/bin/kcov"

install() {
    if [ ! -f "$KCOV" ]; then
        wget "https://github.com/SimonKagstrom/kcov/archive/master.tar.gz"
        tar xzf "master.tar.gz"
        mkdir "kcov-master/build"
        cd "kcov-master/build"
        cmake ..
        make && make install DESTDIR="../../$KCOV_BUILD"
        cd ../../
        rm -rf kcov-master
    fi
}

run() {
    rm -rf "$KCOV_OUT"

    local files
    files=$(cargo test --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]")

    local target_dir
    for file in ${files}; do
        target_dir="$KCOV_OUT/$(basename "$file")"
        mkdir -p "$target_dir"
        "$KCOV" --exclude-pattern=/.cargo,/usr/lib --verify "$target_dir" "$file"
    done
}

upload() {
    bash <(curl -s https://codecov.io/bash)
}

install && \
 run && \
 upload
