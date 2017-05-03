//! A module for working with the [neko vm](http://nekovm.org/).
//!
//! This module contains two main parts.
//!
//! 1. The raw FFI bindings in the [raw submodule](raw/).
//! 2. A safe, rusty wrapper in the top level.
//!
//! Unless you have a specific reason otherwise you should always use the safe wrapper instead of the raw bindings.
pub mod raw {
    //! This module contains the raw neko bindings generated using bindgen.
    //! See http://nekovm.org/doc/vm for more information.
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
