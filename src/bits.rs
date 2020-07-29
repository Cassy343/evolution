use std::num::Wrapping;
use std::ops::*;

pub trait BitString:
    Clone +
    BitAnd<Output=Self> + 
    BitAndAssign +
    BitOr<Output=Self> +
    BitOrAssign +
    BitXor<Output=Self> +
    BitXorAssign +
    Not<Output=Self>
{
    fn len(&self) -> usize;

    fn flip(&mut self, index: usize);

    // Returns old value
    fn flip_get(&mut self, index: usize) -> bool;

    fn substring(&self, from: usize, to: usize) -> Self;
}

impl BitString for u32 {
    #[inline(always)]
    fn len(&self) -> usize {
        32
    }

    #[inline]
    fn flip(&mut self, index: usize) {
        *self ^= 1u32 << index;
    }

    #[inline]
    fn flip_get(&mut self, index: usize) -> bool {
        let mask = 1u32 << index;
        *self ^= mask;
        *self & mask == 0
    }

    #[inline]
    fn substring(&self, from: usize, to: usize) -> Self {
        let mask = ((Wrapping(1u32) << to) - Wrapping(1u32)) ^ ((Wrapping(1u32) << from) - Wrapping(1u32));
        *self & mask.0
    }
}

impl BitString for u64 {
    #[inline(always)]
    fn len(&self) -> usize {
        64
    }

    #[inline]
    fn flip(&mut self, index: usize) {
        *self ^= 1u64 << index;
    }

    #[inline]
    fn flip_get(&mut self, index: usize) -> bool {
        let mask = 1u64 << index;
        *self ^= mask;
        *self & mask == 0
    }

    #[inline]
    fn substring(&self, from: usize, to: usize) -> Self {
        let mask = ((Wrapping(1u64) << to) - Wrapping(1u64)) ^ ((Wrapping(1u64) << from) - Wrapping(1u64));
        *self & mask.0
    }
}

unsafe impl<T: BitString> BitStringRepr for T {
    type Repr = T;

    #[inline(always)]
    fn as_bit_string(&self) -> &Self::Repr {
        self
    }

    #[inline(always)]
    fn as_mut_bit_string(&mut self) -> &mut Self::Repr {
        self
    }
}

pub unsafe trait BitStringRepr: Sized {
    type Repr: BitString;

    fn as_bit_string(&self) -> &Self::Repr {
        unsafe { &*(self as *const Self as *const Self::Repr) }
    }

    fn as_mut_bit_string(&mut self) -> &mut Self::Repr {
        unsafe { &mut *(self as *mut Self as *mut Self::Repr) }
    }
}

unsafe impl BitStringRepr for f32 {
    type Repr = u32;
}

unsafe impl BitStringRepr for i32 {
    type Repr = u32;
}

unsafe impl BitStringRepr for f64 {
    type Repr = u64;
}

unsafe impl BitStringRepr for i64 {
    type Repr = u64;
}