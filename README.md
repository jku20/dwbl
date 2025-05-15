# dwbl
Rust binding for Yosys's RTLIL. This lets you manipulate RTLIL designs in Rust. These programs can be neat in their own right, but if desired, they can be compiled into static libraries and linked right back into Yosys plugins written in C++. dwbl uses [cxx](cxx.rs) to build generate the bindings. 

## Usage
TODO: Make sure this actually works.

This library should work as a dependency out of the box using against whatever version of Yosys is in the submodule.

One cool use case which takes a bit more thought is linking the Rust code back into C++, for example, if you wanted to use it in a Yosys plugin. A good way to do this is using [cxx](cxx.rs) to generate bindings. Set the crate-type to "staticlib" to generate an archive by add the following to your Cargo.toml:
```toml
[lib]
crate-type = ["staticlib"]
```
where `lib` is the module with your bindings.

Running `cargo build` should should be place an archive and bindings in `<project-root>/<target>/<profile>`. For debug builds this is normally `<project-root>/target/debug` and for release builds `<project-root>/target/release`. When creating a plugin using `yosys-config`, make sure to use the same version of `yosys` as used by `dwbl`, and then link against this library when generating the plugin. This is normally with the following command:
```bash
yosys-config --exec --cxx --cxxflags --ldflags -o <plugin-output.so> -shared <plugin-source.cc> --libs -L<dir-with-your-archive> -l<archive-name-without-preceding-lib>
```

## Testing
TODO: What is the scheme for testing this?
