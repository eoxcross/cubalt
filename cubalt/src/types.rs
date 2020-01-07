#![allow(non_camel_case_types)]
use std::arch::x86_64::{__m128i,__m256i};

pub type m128i = __m128i;
pub type m256i = __m256i;

pub struct Eori(pub i32);
pub struct Cori(pub i32);
