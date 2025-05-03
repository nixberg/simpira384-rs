#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m128i, _mm_aesenc_si128, _mm_loadu_si128, _mm_setr_epi32, _mm_storeu_si128,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, _mm_aesenc_si128, _mm_loadu_si128, _mm_setr_epi32, _mm_storeu_si128,
};

#[inline]
pub fn permute_in_place(bytes: &mut [u8; 48]) {
    let mut state = [
        unsafe { _mm_loadu_si128(bytes.as_ptr().offset(00).cast()) },
        unsafe { _mm_loadu_si128(bytes.as_ptr().offset(16).cast()) },
        unsafe { _mm_loadu_si128(bytes.as_ptr().offset(32).cast()) },
    ];

    round::<0, 1, 2, 01>(&mut state);
    round::<1, 2, 0, 02>(&mut state);
    round::<2, 0, 1, 03>(&mut state);
    round::<0, 1, 2, 04>(&mut state);
    round::<1, 2, 0, 05>(&mut state);
    round::<2, 0, 1, 06>(&mut state);
    round::<0, 1, 2, 07>(&mut state);
    round::<1, 2, 0, 08>(&mut state);
    round::<2, 0, 1, 09>(&mut state);
    round::<0, 1, 2, 10>(&mut state);
    round::<1, 2, 0, 11>(&mut state);
    round::<2, 0, 1, 12>(&mut state);
    round::<0, 1, 2, 13>(&mut state);
    round::<1, 2, 0, 14>(&mut state);
    round::<2, 0, 1, 15>(&mut state);
    round::<0, 1, 2, 16>(&mut state);
    round::<1, 2, 0, 17>(&mut state);
    round::<2, 0, 1, 18>(&mut state);
    round::<0, 1, 2, 19>(&mut state);
    round::<1, 2, 0, 20>(&mut state);
    round::<2, 0, 1, 21>(&mut state);

    unsafe {
        _mm_storeu_si128(bytes.as_mut_ptr().offset(16).cast(), state[1]);
        _mm_storeu_si128(bytes.as_mut_ptr().offset(00).cast(), state[0]);
        _mm_storeu_si128(bytes.as_mut_ptr().offset(32).cast(), state[2]);
    }
}

#[inline(always)]
fn round<const X: usize, const Y: usize, const Z: usize, const I: i32>(state: &mut [__m128i; 3]) {
    unsafe {
        let constant = _mm_setr_epi32(0x03 ^ I, 0x13 ^ I, 0x23 ^ I, 0x33 ^ I);
        state[Y] = _mm_aesenc_si128(_mm_aesenc_si128(state[X], constant), state[Y]);
    }
}
