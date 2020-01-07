extern crate simdeez;

pub mod types;
#[macro_use]
pub mod macros;
pub mod sse;
pub mod avx2;

pub fn toplevel() {
    println!("toplevel: hi");
    println!("{:?}", sse::identity());
}

