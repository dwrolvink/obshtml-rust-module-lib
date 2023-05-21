# Introduction
This is an example project for demonstrating using rust in an obsidianhtml module.
The current implementation is an extremely crude POC, and is just here to show the basic concept.

# Compiling
Before you can run this example, you will probably need to compile the rust code, unless you are on the same kind of machine as me.
The rust code in question is `src/hello.rs`.

To compile the code, first install rust along with cargo:
- https://www.rust-lang.org/tools/install

Then, run:
``` bash
cargo build --release
```

This will compile `hello` (or `hello.exe`, if you are on Windows) at path `./target/release/hello`.
The `target` folder is gitignored by default, and it's easiest to keep it like this.

To make the compiled binary persist, copy it to `obsidianhtml_rust_module_example/src/hello`.
The python code will execute the binary from that location, so if you skip this step, you will not effectively update the binary after making changes!

On linux you can run `./build`, it combines the two steps above.

# Running
To test your module, you can run:
``` bash
python test.py
```

This will import and instantiate the `ObsidianHtmlRustExampleModule` and run its `run()` method the same way that ObsidianHtml will.
Make sure that the dummy path used in the run method (see `obsidianhtml_rust_module_example/module.py`) is a valid path for your system.