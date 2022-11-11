# list available recipes
default:
	@just -l

# install the dependencies (requires root)
install-deps:
	sudo npm i -g -D postcss postcss-cli
	sudo npm i -g autoprefixer sass

# deploy the web app and docs on the RasPi
deploy: web-deploy doc-deploy

# deploy the web app on the RasPi
web-deploy:
	#!/usr/bin/env bash
	mkdir -p /data/www/wordle

	cd {{justfile_directory()}}/web
	git restore Trunk.toml

	echo ''                          >> Trunk.toml
	echo '[build]'                   >> Trunk.toml
	echo 'dist = "/data/www/wordle"' >> Trunk.toml
	echo 'public_url = "/wordle/"'   >> Trunk.toml
	echo 'release = true'            >> Trunk.toml

	trunk build
	git restore Trunk.toml
	cd {{justfile_directory()}}

# build the docs and optionally open them
doc-build open='':
	cargo doc --no-deps --document-private-items --workspace --release --target-dir target {{open}}

# build and open the docs
doc-open: (doc-build "--open")

# deploy the docs on the RasPi
doc-deploy: doc-build
	mkdir -p /data/www/wordle
	mv target/doc/ /data/www/tictactoe/doc
