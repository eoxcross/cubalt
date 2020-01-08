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
pub struct Cube(m256i);

/// The low 128-bit lane of the m256 that stores edge state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Edges(m128i);

/// The high 128-bit lane of the m256 that stores corner state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Corners(m128i);

/// A single edge state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Edge(u8);

/// A single corner state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Corner(u8);

impl Cube {
    pub fn new(v: m256i) -> Self {
        Self(v)
    }

    /// __m128i ev() const
    #[inline(always)]
    fn edges(&self) -> &Edges {
        unsafe { 
            let arr = std::mem::transmute::<&m256i, &[m128i; 2]>(&self.0);
            let ret = std::mem::transmute::<&m128i, &Edges>(&arr[0]);
            ret
        }
    }

    /// __m128i& ev()
    #[inline(always)]
    fn edges_mut(&mut self) -> &mut Edges {
        unsafe { 
            let arr = std::mem::transmute::<&mut m256i, &mut [m128i; 2]>(&mut self.0);
            let ret = std::mem::transmute::<&mut m128i, &mut Edges>(&mut arr[0]);
            ret
        }
    }

    /// __m128i cv() const
    #[inline(always)]
    fn corners(&self) -> &Corners {
        unsafe { 
            let arr = std::mem::transmute::<&m256i, &[m128i; 2]>(&self.0);
            let ret = std::mem::transmute::<&m128i, &Corners>(&arr[1]);
            ret
        }
    }

    /// __m128i cv() const
    #[inline(always)]
    fn corners_mut(&mut self) -> &mut Corners {
        unsafe { 
            let arr = std::mem::transmute::<&mut m256i, &mut [m128i; 2]>(&mut self.0);
            let ret = std::mem::transmute::<&mut m128i, &mut Corners>(&mut arr[1]);
            ret
        }
    }

    /// A mutable reference to the low half of the m128 that actually stores corner state.
    /// u64()[2]
    fn corners_64_mut(&mut self) -> &mut u64 {
        let arr = unsafe { std::mem::transmute::<&mut Cube, &mut [u64; 4]>(self) };
        &mut arr[2]
    }
}
