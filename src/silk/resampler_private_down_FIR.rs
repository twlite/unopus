use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn silk_resampler_private_AR2(
            S: *mut i32,
            out_Q8: *mut i32,
            in_0: *const i16,
            A_Q14: *const i16,
            len: i32,
        );
    }
}
use self::resampler_private_h::silk_resampler_private_AR2;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::celt::celt::celt_fatal;
use crate::externs::memcpy;
use crate::silk::resampler_rom::{
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};
use crate::silk::resampler_structs::silk_resampler_state_struct;

#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn silk_resampler_private_down_FIR_INTERPOL(
    mut out: *mut i16,
    buf: *mut i32,
    FIR_Coefs: *const i16,
    FIR_Order: libc::c_int,
    FIR_Fracs: libc::c_int,
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> *mut i16 {
    let mut index_Q16: i32 = 0;
    let mut res_Q6: i32 = 0;
    let mut buf_ptr: *mut i32 = 0 as *mut i32;
    let mut interpol_ind: i32 = 0;
    let mut interpol_ptr: *const i16 = 0 as *const i16;
    match FIR_Order {
        RESAMPLER_DOWN_ORDER_FIR0 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                interpol_ind = ((index_Q16 & 0xffff as libc::c_int) as libc::c_long
                    * FIR_Fracs as i16 as i64
                    >> 16 as libc::c_int) as i32;
                interpol_ptr = &*FIR_Coefs
                    .offset((RESAMPLER_DOWN_ORDER_FIR0 / 2 as libc::c_int * interpol_ind) as isize)
                    as *const i16;
                res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                    * *interpol_ptr.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(1 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(2 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(3 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(4 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(4 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(5 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(5 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(6 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(6 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(7 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(7 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(8 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(8 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                interpol_ptr = &*FIR_Coefs.offset(
                    (RESAMPLER_DOWN_ORDER_FIR0 / 2 as libc::c_int
                        * (FIR_Fracs - 1 as libc::c_int - interpol_ind))
                        as isize,
                ) as *const i16;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(17 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(0 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(16 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(1 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(15 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(2 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(14 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(3 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(13 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(4 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(12 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(5 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(11 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(6 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(10 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(7 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + (*buf_ptr.offset(9 as libc::c_int as isize) as libc::c_long
                        * *interpol_ptr.offset(8 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR1 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as libc::c_int as isize)
                    + *buf_ptr.offset(23 as libc::c_int as isize))
                    as libc::c_long
                    * *FIR_Coefs.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(1 as libc::c_int as isize)
                        + *buf_ptr.offset(22 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(1 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(2 as libc::c_int as isize)
                        + *buf_ptr.offset(21 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(2 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(3 as libc::c_int as isize)
                        + *buf_ptr.offset(20 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(3 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(4 as libc::c_int as isize)
                        + *buf_ptr.offset(19 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(4 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(5 as libc::c_int as isize)
                        + *buf_ptr.offset(18 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(5 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(6 as libc::c_int as isize)
                        + *buf_ptr.offset(17 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(6 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(7 as libc::c_int as isize)
                        + *buf_ptr.offset(16 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(7 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(8 as libc::c_int as isize)
                        + *buf_ptr.offset(15 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(8 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(9 as libc::c_int as isize)
                        + *buf_ptr.offset(14 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(9 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(10 as libc::c_int as isize)
                        + *buf_ptr.offset(13 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(10 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(11 as libc::c_int as isize)
                        + *buf_ptr.offset(12 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(11 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                let fresh1 = out;
                out = out.offset(1);
                *fresh1 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR2 => {
            index_Q16 = 0 as libc::c_int;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as libc::c_int) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as libc::c_int as isize)
                    + *buf_ptr.offset(35 as libc::c_int as isize))
                    as libc::c_long
                    * *FIR_Coefs.offset(0 as libc::c_int as isize) as i64
                    >> 16 as libc::c_int) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(1 as libc::c_int as isize)
                        + *buf_ptr.offset(34 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(1 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(2 as libc::c_int as isize)
                        + *buf_ptr.offset(33 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(2 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(3 as libc::c_int as isize)
                        + *buf_ptr.offset(32 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(3 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(4 as libc::c_int as isize)
                        + *buf_ptr.offset(31 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(4 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(5 as libc::c_int as isize)
                        + *buf_ptr.offset(30 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(5 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(6 as libc::c_int as isize)
                        + *buf_ptr.offset(29 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(6 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(7 as libc::c_int as isize)
                        + *buf_ptr.offset(28 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(7 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(8 as libc::c_int as isize)
                        + *buf_ptr.offset(27 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(8 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(9 as libc::c_int as isize)
                        + *buf_ptr.offset(26 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(9 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(10 as libc::c_int as isize)
                        + *buf_ptr.offset(25 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(10 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(11 as libc::c_int as isize)
                        + *buf_ptr.offset(24 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(11 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(12 as libc::c_int as isize)
                        + *buf_ptr.offset(23 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(12 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(13 as libc::c_int as isize)
                        + *buf_ptr.offset(22 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(13 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(14 as libc::c_int as isize)
                        + *buf_ptr.offset(21 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(14 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(15 as libc::c_int as isize)
                        + *buf_ptr.offset(20 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(15 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(16 as libc::c_int as isize)
                        + *buf_ptr.offset(19 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(16 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                res_Q6 = (res_Q6 as libc::c_long
                    + ((*buf_ptr.offset(17 as libc::c_int as isize)
                        + *buf_ptr.offset(18 as libc::c_int as isize))
                        as libc::c_long
                        * *FIR_Coefs.offset(17 as libc::c_int as isize) as i64
                        >> 16 as libc::c_int)) as i32;
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as libc::c_int == 1 as libc::c_int {
                    (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
                } else {
                    (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                        >> 1 as libc::c_int
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        _ => {
            if 0 as libc::c_int == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const libc::c_char,
                    b"silk/resampler_private_down_FIR.c\0" as *const u8 as *const libc::c_char,
                    139 as libc::c_int,
                );
            }
        }
    }
    return out;
}
#[no_mangle]
#[c2rust::src_loc = "145:1"]
pub unsafe extern "C" fn silk_resampler_private_down_FIR(
    SS: *mut libc::c_void,
    mut out: *mut i16,
    mut in_0: *const i16,
    mut inLen: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: i32 = 0;
    let mut max_index_Q16: i32 = 0;
    let mut index_increment_Q16: i32 = 0;
    let mut FIR_Coefs: *const i16 = 0 as *const i16;
    let vla = ((*S).batchSize + (*S).FIR_Order) as usize;
    let mut buf: Vec<i32> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR.i32_0).as_mut_ptr() as *const libc::c_void,
        ((*S).FIR_Order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    FIR_Coefs = &*((*S).Coefs).offset(2 as libc::c_int as isize) as *const i16;
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        silk_resampler_private_AR2(
            ((*S).sIIR).as_mut_ptr(),
            &mut *buf.as_mut_ptr().offset((*S).FIR_Order as isize),
            in_0,
            (*S).Coefs,
            nSamplesIn,
        );
        max_index_Q16 = ((nSamplesIn as u32) << 16 as libc::c_int) as i32;
        out = silk_resampler_private_down_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            FIR_Coefs,
            (*S).FIR_Order,
            (*S).FIR_Fracs,
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 1 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const libc::c_void,
            ((*S).FIR_Order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR.i32_0).as_mut_ptr() as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const libc::c_void,
        ((*S).FIR_Order as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
