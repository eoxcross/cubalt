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
///   E = edge index (0..=11)
///
/// Corner values (8 bits):
///   --OO-CCC
///   - = unused (zero)
///   O = orientation (0..=2)
///   C = corner index (0..=7)
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Cube(pub m256i);

/// The low 128-bit lane of the m256 that stores edge state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EdgeLane(m128i);

/// The high 128-bit lane of the m256 that stores corner state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CornerLane(m128i);

/// A single edge state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Edge(pub u8);

/// A single corner state.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Corner(pub u8);

impl Cube {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(avx2::identity())
    }

    pub fn new(corners: u64, edges_high: u64, edges_low: u64) -> Self {
        Self(unsafe {
            avx2::literal(
                std::mem::transmute(corners),
                std::mem::transmute(edges_high),
                std::mem::transmute(edges_low),
            )
        })
    }

    fn from_raw_m256(v: m256i) -> Self {
        Self(v)
    }

    #[inline(always)]
    pub fn parity(&self) -> bool {
        avx2::parity(self.0)
    }

    /// uint8_t *edge = reinterpret_cast<uint8_t*>(&ev());
    #[inline(always)]
    pub fn edges(&self) -> &[Edge] {
        unsafe {
            let edge_lane = self.edge_lane();
            let edge_arr =
                std::mem::transmute::<&EdgeLane, &[Edge; 16]>(&edge_lane);
            &edge_arr[0..=11]
        }
    }

    /// uint8_t *edge = reinterpret_cast<uint8_t*>(&ev());
    #[inline(always)]
    pub fn edges_mut(&mut self) -> &mut [Edge] {
        unsafe {
            let mut edge_lane = self.edge_lane_mut();
            let edge_arr = std::mem::transmute::<&mut EdgeLane, &mut [Edge; 16]>(
                &mut edge_lane,
            );
            &mut edge_arr[0..=11]
        }
    }

    /// __m128i ev() const
    #[inline(always)]
    pub fn edge_lane(&self) -> &EdgeLane {
        unsafe {
            let arr = std::mem::transmute::<&m256i, &[m128i; 2]>(&self.0);
            let ret = std::mem::transmute::<&m128i, &EdgeLane>(&arr[0]);
            ret
        }
    }

    /// __m128i& ev()
    #[inline(always)]
    pub fn edge_lane_mut(&mut self) -> &mut EdgeLane {
        unsafe {
            let arr =
                std::mem::transmute::<&mut m256i, &mut [m128i; 2]>(&mut self.0);
            let ret =
                std::mem::transmute::<&mut m128i, &mut EdgeLane>(&mut arr[0]);
            ret
        }
    }

    /// uint8_t *corner = reinterpret_cast<uint8_t*>(&cv());
    #[inline(always)]
    pub fn corners(&self) -> &[Corner] {
        unsafe {
            // doesn't work
            // let corner_lane = self.corner_lane().0;
            let corner_lane = self.corner_lane();
            let corner_arr =
                std::mem::transmute::<&CornerLane, &[Corner; 16]>(&corner_lane);
            &corner_arr[0..=7]
        }
    }

    /// uint8_t *corner = reinterpret_cast<uint8_t*>(&cv());
    #[inline(always)]
    pub fn corners_mut(&mut self) -> &mut [Corner] {
        unsafe {
            let mut corner_lane = self.corner_lane_mut();
            let corner_arr = std::mem::transmute::<
                &mut CornerLane,
                &mut [Corner; 16],
            >(&mut corner_lane);
            &mut corner_arr[0..=7]
        }
    }

    /// __m128i cv() const
    #[inline(always)]
    pub fn corner_lane(&self) -> &CornerLane {
        unsafe {
            let arr = std::mem::transmute::<&m256i, &[m128i; 2]>(&self.0);
            let ret = std::mem::transmute::<&m128i, &CornerLane>(&arr[1]);
            ret
        }
    }

    /// __m128i cv() const
    #[inline(always)]
    pub fn corner_lane_mut(&mut self) -> &mut CornerLane {
        unsafe {
            let arr =
                std::mem::transmute::<&mut m256i, &mut [m128i; 2]>(&mut self.0);
            let ret =
                std::mem::transmute::<&mut m128i, &mut CornerLane>(&mut arr[1]);
            ret
        }
    }

    /// A mutable reference to the low half of the m128 that actually stores
    /// corner state.
    /// u64()[2]
    pub fn corners_64_mut(&mut self) -> &mut u64 {
        let arr =
            unsafe { std::mem::transmute::<&mut Cube, &mut [u64; 4]>(self) };
        &mut arr[2]
    }
}
