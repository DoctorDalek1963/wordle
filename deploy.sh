#!/usr/bin/env bash

# Install dependencies:
# sudo npm i -g -D postcss postcss-cli
# sudo npm i -g autoprefixer sass

mkdir -p /data/www/wordle/${WORDLE_NAME}

# Web app
cd web
git restore Trunk.toml

echo '\n[build]'                                >> Trunk.toml
echo 'dist = "/data/www/wordle/${WORDLE_NAME}"' >> Trunk.toml
echo 'public_url = "/wordle/${WORDLE_NAME}/"'   >> Trunk.toml
echo 'release = true'                           >> Trunk.toml

trunk build
git restore Trunk.toml

# Docs
#cd ..
#cargo doc --no-deps --document-private-items --workspace --release --target-dir target
#mv target/doc/ /data/www/wordle/doc