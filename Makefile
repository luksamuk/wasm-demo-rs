# Please notice that this is not really a very well-structured Makefile.

# If you just want to debug and run a live server, run the command:
#     $ cargo +nightly web start --target-webasm --release
# Do not try to compile this on debug builds, the feature is currently broken.
# I have successfully seen that this works if you run `lite-server` (you can
# install it using `npm`) on the site/ folder.


# The original name of the project changed, so I thought I should
# centralize the project name. If you change the binary name in
# Cargo.toml, update this.
name=wasm-demo-rs



# Those targets theoretically produce no files.
# That's not really true here, but...
.PHONY: folder wasm webstart clean

# To build, we need to create folders, copy our index.html,
# and also copy our wasm and js files.
all: folder site/index.html site/$(name).wasm




# Producing the html file is just a matter of copying
# the static index.html file and replacing "js/app.js" (for cargo web
# start) with the actual name of the js file.
# Notice that we also copy whatever is in the static/ folder, for
# convenience.
site/index.html: static/index.html
	cp static/* site/
	find site/index.html -type f -exec sed -i 's/js\/app.js/$(name).js/g' {} +

# After building our wasm files, we copy them to the output folder.
site/$(name).wasm: wasm
	cp target/wasm32-unknown-unknown/release/*.wasm site/
	cp target/wasm32-unknown-unknown/release/*.js site/




# Building the wasm files requires cargo web, which we have installed with the
# nightly toolchain.
wasm: src/main.rs
	cargo web build --target wasm32-unknown-unknown --release




# Folder creation magic.
folder:
	mkdir -p site


# Though using the Makefile to handle this is not really something very handy,
# you can run `make webstart` to run the server, which will also watch for code
# changes to execute compilation.
webstart:
	cargo web start --target wasm32-unknown-unknown --release

clean:
	cargo clean
