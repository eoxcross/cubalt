#![cfg(all(
    target_feature = "sse",
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "avx",
    target_feature = "avx2",
    target_arch = "x86_64",
))]
#![allow(dead_code)]
pub mod types;
#[macro_use]
pub mod macros;
pub mod avx2;
pub mod cube;
pub mod sse;

pub fn toplevel() {
    println!("toplevel: hi");
    let cube = cube::Cube::identity();
    println!("{:?}", cube.0);
    for edge in cube.edges() {
        print!("{:?} | ", edge.0);
    }
    println!();
    for corner in cube.corners() {
        print!("{:?} | ", corner.0);
    }
    println!();
    let cube_inv = !cube;
    println!("{:?}", cube_inv.0);
    for edge in cube_inv.edges() {
        print!("{:?} | ", edge.0);
    }
    println!();
    for corner in cube_inv.corners() {
        print!("{:?} | ", corner.0);
    }
    println!();
}
