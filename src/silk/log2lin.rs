#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "42:9"]
    pub const silk_int32_MAX: i32 = 0x7fffffff as i32;
}

pub use self::typedef_h::silk_int32_MAX;

#[c2rust::src_loc = "36:1"]
pub unsafe fn silk_log2lin(inLog_Q7: i32) -> i32 {
    let mut out: i32 = 0;
    let mut frac_Q7: i32 = 0;
    if inLog_Q7 < 0 as i32 {
        return 0 as i32;
    } else {
        if inLog_Q7 >= 3967 as i32 {
            return silk_int32_MAX;
        }
    }
    out = ((1 as i32 as u32) << (inLog_Q7 >> 7 as i32)) as i32;
    frac_Q7 = inLog_Q7 & 0x7f as i32;
    if inLog_Q7 < 2048 as i32 {
        out = out
            + (out
                * (frac_Q7 as i64
                    + ((frac_Q7 as i16 as i32 * (128 as i32 - frac_Q7) as i16 as i32) as i64
                        * -(174 as i32) as i16 as i64
                        >> 16 as i32)) as i32
                >> 7 as i32);
    } else {
        out = out
            + (out >> 7 as i32)
                * (frac_Q7 as i64
                    + ((frac_Q7 as i16 as i32 * (128 as i32 - frac_Q7) as i16 as i32) as i64
                        * -(174 as i32) as i16 as i64
                        >> 16 as i32)) as i32;
    }
    return out;
}
