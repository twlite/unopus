use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
use crate::silk::define::STEREO_INTERP_LEN_MS;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::silk::structs::stereo_dec_state;

use crate::externs::memcpy;
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_MS_to_LR(
    mut state: *mut stereo_dec_state,
    x1: *mut i16,
    x2: *mut i16,
    pred_Q13: *const i32,
    fs_kHz: libc::c_int,
    frame_length: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut denom_Q16: libc::c_int = 0;
    let mut delta0_Q13: libc::c_int = 0;
    let mut delta1_Q13: libc::c_int = 0;
    let mut sum: i32 = 0;
    let mut diff: i32 = 0;
    let mut pred0_Q13: i32 = 0;
    let mut pred1_Q13: i32 = 0;
    memcpy(
        x1 as *mut libc::c_void,
        ((*state).sMid).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        x2 as *mut libc::c_void,
        ((*state).sSide).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sMid).as_mut_ptr() as *mut libc::c_void,
        &mut *x1.offset(frame_length as isize) as *mut i16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sSide).as_mut_ptr() as *mut libc::c_void,
        &mut *x2.offset(frame_length as isize) as *mut i16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    pred0_Q13 = (*state).pred_prev_Q13[0 as libc::c_int as usize] as i32;
    pred1_Q13 = (*state).pred_prev_Q13[1 as libc::c_int as usize] as i32;
    denom_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) / (8 as libc::c_int * fs_kHz);
    delta0_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(0 as libc::c_int as isize)
                - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
                as i16 as i32
                * denom_Q16 as i16 as i32
                & 1 as libc::c_int)
    } else {
        ((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    delta1_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(1 as libc::c_int as isize)
                - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
                as i16 as i32
                * denom_Q16 as i16 as i32
                & 1 as libc::c_int)
    } else {
        ((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int) as i16
            as i32
            * denom_Q16 as i16 as i32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    n = 0 as libc::c_int;
    while n < STEREO_INTERP_LEN_MS * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int) as i32)
            as u32)
            << 9 as libc::c_int) as i32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 8 as libc::c_int)
            as i32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        sum = (sum as libc::c_long
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 11 as libc::c_int)
                as i32 as libc::c_long
                * pred1_Q13 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        n += 1;
    }
    pred0_Q13 = *pred_Q13.offset(0 as libc::c_int as isize);
    pred1_Q13 = *pred_Q13.offset(1 as libc::c_int as isize);
    n = STEREO_INTERP_LEN_MS * fs_kHz;
    while n < frame_length {
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as u32) << 1 as libc::c_int) as i32)
            as u32)
            << 9 as libc::c_int) as i32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 8 as libc::c_int)
            as i32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        sum = (sum as libc::c_long
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as i32 as u32) << 11 as libc::c_int)
                as i32 as libc::c_long
                * pred1_Q13 as i16 as i64
                >> 16 as libc::c_int)) as i32;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        n += 1;
    }
    (*state).pred_prev_Q13[0 as libc::c_int as usize] =
        *pred_Q13.offset(0 as libc::c_int as isize) as i16;
    (*state).pred_prev_Q13[1 as libc::c_int as usize] =
        *pred_Q13.offset(1 as libc::c_int as isize) as i16;
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            + *x2.offset((n + 1 as libc::c_int) as isize) as i32;
        diff = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            - *x2.offset((n + 1 as libc::c_int) as isize) as i32;
        *x1.offset((n + 1 as libc::c_int) as isize) = (if sum > silk_int16_MAX {
            silk_int16_MAX
        } else if sum < silk_int16_MIN {
            silk_int16_MIN
        } else {
            sum
        }) as i16;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if diff > silk_int16_MAX {
            silk_int16_MAX
        } else if diff < silk_int16_MIN {
            silk_int16_MIN
        } else {
            diff
        }) as i16;
        n += 1;
    }
}
