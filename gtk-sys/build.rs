// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
extern crate system_deps;

#[cfg(not(feature = "dox"))]
use std::io;
#[cfg(not(feature = "dox"))]
use std::io::prelude::*;
#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}
