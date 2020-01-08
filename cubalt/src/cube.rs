use crate::avx2;
use crate::sse;
use crate::types::*;

/// The basic SIMD-friendly cube representation.
///
/// Low 128-bit lane:
///   4 U-face edges
///   4 D-face edges
///   4 E-slice edges
///   4 (unused)
///
/// High 128-bit lane:
///   4 U-face corners
///   4 D-face corners
///   8 (unused)
///
/// Edge values (8 bits):
///   ---OEEEE
///   - = unused (zero)
///   O = orientation
///   E = edge index (0..11)
///
/// Corner values (8 bits):
///   --OO-CCC
///   - = unused (zero)
///   O = orientation (0..2)
///   C = corner index (0..7)
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Cube(pub m256i);

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Edges(pub m128i);

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Corners(pub m128i);

impl Cube {
    pub fn new(v: m256i) -> Self {
        Self(v)
    }

    #[inline(always)]
    fn edges(&self) -> &Edges {
        let arr = unsafe { std::mem::transmute::<&Cube, &[Edges; 2]>(self) };
        &arr[0]
    }

    #[inline(always)]
    fn edges_mut(&mut self) -> &mut Edges {
        let arr = unsafe { std::mem::transmute::<&mut Cube, &mut [Edges; 2]>(self) };
        &mut arr[0]
    }

    #[inline(always)]
    fn corners(&self) -> &Corners {
        let arr = unsafe { std::mem::transmute::<&Cube, &[Corners; 2]>(self) };
        &arr[1]
    }

    #[inline(always)]
    fn corners_mut(&mut self) -> &mut Corners {
        let arr = unsafe { std::mem::transmute::<&mut Cube, &mut [Corners; 2]>(self) };
        &mut arr[1]
    }

    #[inline(always)]
    fn corners_64_mut(&mut self) -> &mut u64 {
        let arr = unsafe { std::mem::transmute::<&mut Cube, &mut [u64; 4]>(self) };
        &mut arr[2]
    }
}
