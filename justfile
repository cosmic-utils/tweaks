name := 'cosmic-ext-tweaks'
export APPID := 'dev.edfloreshz.Tweaks'

rootdir := ''
prefix := '/app'

base-dir := absolute_path(clean(rootdir / prefix))


bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name

desktop := APPID + '.desktop'
desktop-src := 'res' / desktop
desktop-dst := base-dir / 'share' / 'applications' / desktop

metainfo := APPID + '.metainfo.xml'
metainfo-src := 'res' / metainfo
metainfo-dst := base-dir / 'share' / 'metainfo' / metainfo

icons-src := 'res' / 'app_icon.svg'
icons-dst := base-dir / 'share' / 'icons' / 'hicolor'

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

dev *args:
    cargo fmt
    just run {{args}}

# Run with debug logs
run *args:
    env RUST_LOG=cosmic_tasks=info RUST_BACKTRACE=full cargo run --release {{args}}

# Installs files
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}
    install -Dm0644 {{metainfo-src}} {{metainfo-dst}}
    install -Dm0644 {{icons-src}} "{{icons-dst}}/scalable/apps/{{APPID}}.svg"

# Uninstalls installed files
uninstall:
    rm {{bin-dst}}
    rm {{desktop-dst}}
    rm {{metainfo-dst}}
    rm "{{icons-dst}}/scalable/apps/{{APPID}}.svg"

# Vendor dependencies locally
vendor:
    #!/usr/bin/env bash
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    echo >> .cargo/config.toml
    echo '[env]' >> .cargo/config.toml
    if [ -n "${SOURCE_DATE_EPOCH}" ]
    then
        source_date="$(date -d "@${SOURCE_DATE_EPOCH}" "+%Y-%m-%d")"
        echo "VERGEN_GIT_COMMIT_DATE = \"${source_date}\"" >> .cargo/config.toml
    fi
    if [ -n "${SOURCE_GIT_HASH}" ]
    then
        echo "VERGEN_GIT_SHA = \"${SOURCE_GIT_HASH}\"" >> .cargo/config.toml
    fi
    tar pcf vendor.tar .cargo vendor
    rm -rf .cargo vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar


sources-gen:
    python3 flatpak-builder-tools/cargo/flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json

# {
#           "type": "git",
#           "url": "https://github.com/edfloreshz/cosmic-tweaks.git",
#           "commit": "c7edf380580dc682e1048661aed4c2b703e3c794"
#         },

uninstallf:
    flatpak uninstall dev.edfloreshz.Tweaks -y || true

# deps: flatpak-builder git-lfs
build-and-install: uninstallf
    flatpak-builder \
        --force-clean \
        --verbose \
        --ccache \
        --user --install \
        --install-deps-from=flathub \
        --repo=repo \
        flatpak-out \
        dev.edfloreshz.Tweaks.json

runf:
    flatpak run dev.edfloreshz.Tweaks