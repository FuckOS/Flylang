pub static TITLE: &str = r#"
 ______  _         _                           
|  ____|| |       | |                          
| |__   | | _   _ | |       __ _  _ __    __ _ 
|  __|  | || | | || |      / _` || '_ \  / _` |
| |     | || |_| || |____ | (_| || | | || (_| |
|_|     |_| \__, ||______| \__,_||_| |_| \__, |
             __/ |                        __/ |
            |___/                        |___/ 
"#;

pub static PACKAGES: &[&str] = &[
    "std.system",
    "std.types",
    "std.auto",
    "std.fmt",
    "std.console",
    "random",
    "vector",
    "collections",
    "fs.events",
    "path",
    "net.dns",
    "net.tcp",
    "net.udp",
    "net",
    "async",
    "ntapi",
    "libc",
    "ffi",
    "python",
    "nodejs",
    "fgcc",
    "async.runtime"
]; // 22

pub static COMMANDS: &[&str] = &[
    "flyl build .",
    "tsc .",
    "make clean",
    "node-gyp rebuild",
    "gcc bar.c -o bar",
    "cmake build",
    "make all-clean",
    "make test",
    "bindgen build",
    "webpack .",
    "rollup ."
]; // 11


pub static DEFAULT_PKG: &str = r#"version: 1.0
name: Hello"#;