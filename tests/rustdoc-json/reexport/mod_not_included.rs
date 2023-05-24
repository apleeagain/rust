// Regression test for https://github.com/rust-lang/rust/issues/101103

#![feature(no_core)]
#![no_core]

mod m1 {
    pub fn x() {}
}

pub use m1::x;

// @has "$.index[*][?(@.name=='x' && @.inner.function)]"
// @has "$.index[*].inner[?(@.import.name=='x')].import.source" '"m1::x"'
// @!has "$.index[*][?(@.name=='m1')]"
