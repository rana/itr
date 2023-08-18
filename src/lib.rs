use std::ops::Range;

/// Returns a range iterator.
/// 
///     seg=2, lim=6: [0..3, 3..6]
///     seg=2, lim=7: [0..3, 3..6, 6..7]
/// 
/// For odd `len`, the actual segment count is `seg + 1`.
/// 
/// # Arguments
/// 
/// * `seg` - The number of segments to divide the length.
///
/// * `len` - The total number of elements.
pub fn rngs(seg: usize, len: usize) -> RngItr {
    RngItr {
        idx: 0,
        stp: len.saturating_div(seg),
        lim: len,
    }
}

// A range iterator.
#[derive(Debug, Clone)]
pub struct RngItr {
    idx: usize,
    stp: usize,
    lim: usize,
}

impl Iterator for RngItr {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == usize::MAX {
            None
        } else {
            let lim = (self.idx + self.stp).min(self.lim);
            let rng = Range {
                start: self.idx,
                end: lim,
            };
            if lim == self.lim {
                self.idx = usize::MAX;
            } else {
                self.idx += self.stp;
            }
            Some(rng)
        }
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    fn rngs_n() {
        assert_eq!(rngs(2, 6).collect::<Vec<Range<usize>>>(), [0..3, 3..6]);
        assert_eq!(rngs(2, 7).collect::<Vec<Range<usize>>>(), [0..3, 3..6, 6..7]);
    }
}
