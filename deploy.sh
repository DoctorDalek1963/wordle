#!/usr/bin/env sh

mkdir -p /data/www/wordle

# Web app
cd web
rm -f Trunk.toml
touch Trunk.toml

echo '[build]'                    > Trunk.toml
echo 'dist = "/data/www/wordle"' >> Trunk.toml
echo 'public_url = "/wordle/"'   >> Trunk.toml
echo 'release = true'            >> Trunk.toml

trunk build
rm Trunk.toml

# Docs
cd ..
cargo doc --no-deps --document-private-items --workspace --release --target-dir target
mv target/doc/ /data/www/wordle/doc
