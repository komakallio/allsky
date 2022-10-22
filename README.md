# allsky

All the sky, all the time. An all-sky capture software for modern CMOS cameras by ZWO.

## Compiling

Follow the [bindgen instructions](https://rust-lang.github.io/rust-bindgen/requirements.html#windows) to make sure bindgen can use LLVM.

To build the program, just run `cargo build`.

Because the ZWO SDK is dynamically linked with the program, you need to make sure the dynamic library can be found when running the program.
On Linux you can do this by specifying the `LD_LIBRARY_PATH` variable, for example like this:

```bash
LD_LIBRARY_PATH=libasi-sys/vendors/zwo-sdk/linux/lib/armv8 cargo run
```

On Windows you need to copy the ASICamera2.dll from the vendors directory to your current directory, or make sure it is in the `PATH`.