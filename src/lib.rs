//! Utility iterators.

use num::traits::AsPrimitive;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::marker::PhantomData;
use std::{mem, ops::Range};

/// Returns a range iterator.
///
///     seg=2,  lim=6: [0..3, 3..6]
///     seg=2,  lim=7: [0..4, 4..7]
///     seg=4, lim=10: [0..3, 3..6, 6..8, 8..10]
///
/// Ranges may have different lengths depending on the `lim % seg` remainder.
///
/// # Arguments
///
/// * `seg` - The number of segments to divide the length.
///
/// * `lim` - The total number of elements.
pub fn rngs(seg: usize, lim: usize) -> RngItr {
    RngItr {
        idx: 0,
        stp: lim.saturating_div(seg),
        lim,
        stp_adj: lim % seg,
    }
}

// A range iterator.
#[derive(Debug, Clone)]
pub struct RngItr {
    idx: usize,
    stp: usize,
    lim: usize,
    stp_adj: usize,
}
impl Iterator for RngItr {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == usize::MAX {
            None
        } else {
            let adj: usize = if self.stp_adj > 0 {
                self.stp_adj -= 1;
                1
            } else {
                0
            };
            let lim = (self.idx + self.stp + adj).min(self.lim);
            let rng = Range {
                start: self.idx,
                end: lim,
            };
            if lim == self.lim {
                self.idx = usize::MAX;
            } else {
                self.idx += self.stp + adj;
            }
            Some(rng)
        }
    }
}

/// Returns an iterator which generates random integers.
///
/// Generates equal quantities of integers represented
/// by 1-byte, 2-bytes, up to n-bytes.
///
/// Generates an infinite number of integers.
pub fn rnds_eql_byt<T>() -> RndEqlBytItr<T>
where
    T: AsPrimitive<T>,
    usize: num::traits::AsPrimitive<T>,
{
    RndEqlBytItr {
        rng: thread_rng(),
        byt: 0,
        phn: PhantomData,
    }
}
/// An iterator generating random integers.
#[derive(Debug, Clone)]
pub struct RndEqlBytItr<T>
where
    T: AsPrimitive<T>,
    usize: num::traits::AsPrimitive<T>,
{
    rng: ThreadRng,
    byt: usize,
    phn: PhantomData<T>,
}

impl<T> Iterator for RndEqlBytItr<T>
where
    T: AsPrimitive<T>,
    usize: num::traits::AsPrimitive<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // Generate a random integer with `byt + 1` number of bytes.

        // Determine min inclusive integer.
        let lo: usize = if self.byt == 0 {
            0
        } else {
            1 << (self.byt * 8)
        };

        // Determine max inclusive integer.
        // Use u128 to allow shifting (1<<64)-1 for 64-bit integer.
        let hi_inc: usize = ((1u128 << ((self.byt + 1) * 8) as u128) - 1) as usize;

        // Generate the random integer.
        let ret: usize = self.rng.gen_range(lo..=hi_inc);

        // Prepare for the next iteration.
        self.byt = (self.byt + 1) % mem::size_of::<Self::Item>();

        Some(ret.as_())
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    fn rngs_n() {
        assert_eq!(rngs(2, 6).collect::<Vec<Range<usize>>>(), [0..3, 3..6]);
        assert_eq!(
            rngs(2, 7).collect::<Vec<Range<usize>>>(),
            [0..4, 4..7]
        );
        assert_eq!(
            rngs(4, 10).collect::<Vec<Range<usize>>>(),
            [0..3, 3..6, 6..8, 8..10]
        );
    }

    #[test]
    fn rnds_with_eq_byte_u64_n() {
        for (idx, val) in rnds_eql_byt::<u64>().take(16).enumerate() {
            let byt_non_zro_cnt = (idx % mem::size_of::<u64>()) + 1;
            // println!(
            //     "byts:{:?}, byt_non_zro_cnt:{}",
            //     val.to_le_bytes(),
            //     byt_non_zro_cnt
            // );
            for (idx, byt) in val.to_le_bytes().into_iter().enumerate() {
                if idx < byt_non_zro_cnt {
                    assert_ne!(byt, 0);
                } else {
                    assert_eq!(byt, 0);
                }
            }
        }
    }

    #[test]
    fn rnds_with_eq_byte_u32_n() {
        for (idx, val) in rnds_eql_byt::<u32>().take(8).enumerate() {
            let byt_non_zro_cnt = (idx % mem::size_of::<u32>()) + 1;
            // println!(
            //     "byts:{:?}, byt_non_zro_cnt:{}",
            //     val.to_le_bytes(),
            //     byt_non_zro_cnt
            // );
            for (idx, byt) in val.to_le_bytes().into_iter().enumerate() {
                if idx < byt_non_zro_cnt {
                    assert_ne!(byt, 0);
                } else {
                    assert_eq!(byt, 0);
                }
            }
        }
    }

    #[test]
    fn rnds_with_eq_byte_u16_n() {
        for (idx, val) in rnds_eql_byt::<u16>().take(4).enumerate() {
            let byt_non_zro_cnt = (idx % mem::size_of::<u16>()) + 1;
            // println!(
            //     "byts:{:?}, byt_non_zro_cnt:{}",
            //     val.to_le_bytes(),
            //     byt_non_zro_cnt
            // );
            for (idx, byt) in val.to_le_bytes().into_iter().enumerate() {
                if idx < byt_non_zro_cnt {
                    assert_ne!(byt, 0);
                } else {
                    assert_eq!(byt, 0);
                }
            }
        }
    }

    #[test]
    fn rnds_with_eq_byte_u8_n() {
        for (idx, val) in rnds_eql_byt::<u8>().take(2).enumerate() {
            let byt_non_zro_cnt = (idx % mem::size_of::<u8>()) + 1;
            // println!(
            //     "byts:{:?}, byt_non_zro_cnt:{}",
            //     val.to_le_bytes(),
            //     byt_non_zro_cnt
            // );
            for (idx, byt) in val.to_le_bytes().into_iter().enumerate() {
                if idx < byt_non_zro_cnt {
                    assert_ne!(byt, 0);
                } else {
                    assert_eq!(byt, 0);
                }
            }
        }
    }
}
