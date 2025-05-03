use core::arch::aarch64::{
    uint8x16_t, vaeseq_u8, vaesmcq_u8, veorq_u8, vld1q_u8, vld1q_u32, vreinterpretq_u8_u32,
    vst1q_u8,
};

#[inline]
pub fn permute_in_place(bytes: &mut [u8; 48]) {
    let mut state = [
        unsafe { vld1q_u8(bytes.as_ptr().offset(00).cast()) },
        unsafe { vld1q_u8(bytes.as_ptr().offset(16).cast()) },
        unsafe { vld1q_u8(bytes.as_ptr().offset(32).cast()) },
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
        vst1q_u8(bytes.as_mut_ptr().offset(00).cast(), state[0]);
        vst1q_u8(bytes.as_mut_ptr().offset(16).cast(), state[1]);
        vst1q_u8(bytes.as_mut_ptr().offset(32).cast(), state[2]);
    }
}

#[inline(always)]
fn round<const X: usize, const Y: usize, const Z: usize, const I: u32>(
    state: &mut [uint8x16_t; 3],
) {
    unsafe {
        let constant = vreinterpretq_u8_u32(vld1q_u32(
            [0x03 ^ I, 0x13 ^ I, 0x23 ^ I, 0x33 ^ I].as_ptr().cast(),
        ));
        state[Y] = veorq_u8(
            vaesmcq_u8(vaeseq_u8(
                vaesmcq_u8(vaeseq_u8(state[X], core::mem::zeroed())),
                constant,
            )),
            state[Y],
        );
    }
}
