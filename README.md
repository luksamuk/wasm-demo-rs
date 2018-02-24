# wasm-demo-rs
A basic example of Rust compiled to WebAssembly, using Rust's own native tools.

## Changelog
- 02/24/2018 -- Upgraded version of `stdweb` to `0.4.0` and upgraded toolchain to latest `nightly`.
- 02/19/2018 -- Latest `nightly` breaks compilation. Used a `rust-toolchain` file to stop the nightly version at `nightly-2018-01-21`, like the `servo` project..


## About
This is a simple demo for compiling Rust to WebAssembly.
This demo uses the nightly toolchain, `cargo-web`, and the `stdweb` crate.
While this is a new feature, there is some setup that you will need to do so you can build and run this. I recommend doing it on Linux, since we'll be using a single Makefile I wrote to make my life easier, but you could also replicate its commands manually on your favorite OS, assuming it runs Rust, `cargo-web` and `stdweb` fine.

I felt the need to create this repository for future reference, so it is only a simple demo with small considerations. I might add or remove things, if necessary.

## Dependencies
As stated above, this demo depends on the nightly toolchain (for now), and the `cargo-web` tool.

### Notes on toolchain selection
The Nightly toolchain might panic when building anything that uses `stdweb` or possibly `wasm32-unknown-unknown`. On a previous version, I've switched to a nice solution (thanks to Elias from the Rust Brazil community on Telegram for this): we have a `rust-toolchain` file specifying which toolchain we intend to use, and more than that, its specific version.

For this example, you can use `nightly-2018-02-23`, if latest `nightly` fails, so just type in the toolchain name with version like this on the `rust-toolchain` file. Also, you might want to install this specific version of nightly with `rustup`, as instructed below.

You can also open an issue on this repo in case compilation fails so I can check it out.

### Installing the toolchain

Assuming you have `rustup` installed, you can install the nightly toolchain with the following command:

```bash
$ rustup toolchain install nightly
```

After that, you can add Rust's native WASM target with this command:

```bash
$ rustup target add wasm32-unknown-unknown --toolchain nightly
```

(I'm not sure the `--toolchain` is needed here though, but hey, whatever)

And that ought to take care of things on the toolchain side. Now, you'll need to install `cargo-web` (v0.6.8).

```bash
$ cargo install cargo-web
```

That should provide you with the latest version of `cargo-web`.

## Compiling

### Using provided Makefile
I've written a well-documented Makefile that should make stuff easier for compiling.
When you run `make`, a "site" folder will appear, containing an `index.html` file, along with the `wasm` binary and the `js` glue code. This will export the application, ready to be uploaded anywhere you'd like for a live demo.

If you just want to test it, you can run `make webstart`, and your browser will be fired up at `localhost:8000` with your demo app.

### Compiling by hand
You can compile directly using `cargo` as well, only instead of building, you'll have to use `cargo-web` to generate your files.
Fire up your terminal and use the following command on the root folder:

```bash
$ cargo web build --target wasm32-unknown-unknown --release
```
This should give you your .js and .wasm files on `target/wasm32-unknown-unknown/release`.
If you only want to test your demo app, you can use `cargo web start` to run it on `localhost:8000`. This will also fire up your browser:

```bash
$ cargo web start --target wasm32-unknown-unknown --release
```

Notice that changing your Rust files will also recompile your project, but you'll have to refresh the browser page manually so it takes effect.

#### Considerations about this method
Notice that the static `index.html` refers to `js/app.js` instead of `wasm-demo-rs.js`. This is done on purpose, since `cargo web start` asks you to refer to this file on your `html`. To fix this, you can use a small substitution, such as using `sed`, when copying the index.html file to your output folder (this is done on the Makefile, so refer to it for an example).

## Useful links
- Koute's stdweb crate [repository](https://github.com/koute/stdweb) and [documentation](https://docs.rs/stdweb/*/stdweb/).
- [Setting up the native WASM target for Rust](https://www.hellorust.com/setup/wasm-target/)
- [Koute's awesome NES emulator, Pinky, running on the web](https://github.com/koute/pinky/tree/master/pinky-web), which inspired this small demo. Also has bindings for Emscripten and asmjs targets.
- [raphamorim's awesome WASM + Rust tutorial](https://github.com/raphamorim/wasm-and-rust), which covers the Emscripten side of things while also explaining some aspects of WASM.
- [My reimplementation of an old game, Super BrickBreak, in Rust/WASM](https://github.com/luksamuk/super-brickbreak-rs), which uses this very code as a template.
