pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod conv_2d;
mod padding;

pub use conv_2d::fft::Conv2DFftExt;
pub use conv_2d::Conv2DExt;

#[derive(Debug, Clone, Copy)]
pub enum ConvType<const N: usize> {
    Full,
    Same,
    Valid,
    // (pad, stride)
    Custom([usize; N], [usize; N]),
    // (pad, stride)
    Explicit([[usize; 2]; N], [usize; N]),
}

// padding mode. It can be either a single BorderType applied on all sides or a custom tuple of two BorderTypes for (H, W), respectively.
#[derive(Debug, Clone, Copy)]
pub enum PaddingMode<const N: usize, T: num::traits::NumAssign + Copy> {
    Zeros,
    Const(T),
    Reflect,
    Replicate,
    Warp,
    Custom([BorderType<T>; N]),
    Explicit([[BorderType<T>; 2]; N])
}

// padding mode for single dim
#[derive(Debug, Clone, Copy)]
pub enum BorderType<T: num::traits::NumAssign + Copy> {
    Zeros,
    Const(T),
    Reflect,
    Replicate,
    Warp,
}
