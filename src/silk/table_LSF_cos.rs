use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    use super::types_h::__int16_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    use super::stdint_intn_h::int16_t;
}
pub use self::types_h::__int16_t;
pub use self::stdint_intn_h::int16_t;
pub use self::opus_types_h::opus_int16;
#[no_mangle]
#[c2rust::src_loc = "36:18"]
pub static mut silk_LSFCosTab_FIX_Q12: [opus_int16; 129] = [
    8192 as libc::c_int as opus_int16,
    8190 as libc::c_int as opus_int16,
    8182 as libc::c_int as opus_int16,
    8170 as libc::c_int as opus_int16,
    8152 as libc::c_int as opus_int16,
    8130 as libc::c_int as opus_int16,
    8104 as libc::c_int as opus_int16,
    8072 as libc::c_int as opus_int16,
    8034 as libc::c_int as opus_int16,
    7994 as libc::c_int as opus_int16,
    7946 as libc::c_int as opus_int16,
    7896 as libc::c_int as opus_int16,
    7840 as libc::c_int as opus_int16,
    7778 as libc::c_int as opus_int16,
    7714 as libc::c_int as opus_int16,
    7644 as libc::c_int as opus_int16,
    7568 as libc::c_int as opus_int16,
    7490 as libc::c_int as opus_int16,
    7406 as libc::c_int as opus_int16,
    7318 as libc::c_int as opus_int16,
    7226 as libc::c_int as opus_int16,
    7128 as libc::c_int as opus_int16,
    7026 as libc::c_int as opus_int16,
    6922 as libc::c_int as opus_int16,
    6812 as libc::c_int as opus_int16,
    6698 as libc::c_int as opus_int16,
    6580 as libc::c_int as opus_int16,
    6458 as libc::c_int as opus_int16,
    6332 as libc::c_int as opus_int16,
    6204 as libc::c_int as opus_int16,
    6070 as libc::c_int as opus_int16,
    5934 as libc::c_int as opus_int16,
    5792 as libc::c_int as opus_int16,
    5648 as libc::c_int as opus_int16,
    5502 as libc::c_int as opus_int16,
    5352 as libc::c_int as opus_int16,
    5198 as libc::c_int as opus_int16,
    5040 as libc::c_int as opus_int16,
    4880 as libc::c_int as opus_int16,
    4718 as libc::c_int as opus_int16,
    4552 as libc::c_int as opus_int16,
    4382 as libc::c_int as opus_int16,
    4212 as libc::c_int as opus_int16,
    4038 as libc::c_int as opus_int16,
    3862 as libc::c_int as opus_int16,
    3684 as libc::c_int as opus_int16,
    3502 as libc::c_int as opus_int16,
    3320 as libc::c_int as opus_int16,
    3136 as libc::c_int as opus_int16,
    2948 as libc::c_int as opus_int16,
    2760 as libc::c_int as opus_int16,
    2570 as libc::c_int as opus_int16,
    2378 as libc::c_int as opus_int16,
    2186 as libc::c_int as opus_int16,
    1990 as libc::c_int as opus_int16,
    1794 as libc::c_int as opus_int16,
    1598 as libc::c_int as opus_int16,
    1400 as libc::c_int as opus_int16,
    1202 as libc::c_int as opus_int16,
    1002 as libc::c_int as opus_int16,
    802 as libc::c_int as opus_int16,
    602 as libc::c_int as opus_int16,
    402 as libc::c_int as opus_int16,
    202 as libc::c_int as opus_int16,
    0 as libc::c_int as opus_int16,
    -(202 as libc::c_int) as opus_int16,
    -(402 as libc::c_int) as opus_int16,
    -(602 as libc::c_int) as opus_int16,
    -(802 as libc::c_int) as opus_int16,
    -(1002 as libc::c_int) as opus_int16,
    -(1202 as libc::c_int) as opus_int16,
    -(1400 as libc::c_int) as opus_int16,
    -(1598 as libc::c_int) as opus_int16,
    -(1794 as libc::c_int) as opus_int16,
    -(1990 as libc::c_int) as opus_int16,
    -(2186 as libc::c_int) as opus_int16,
    -(2378 as libc::c_int) as opus_int16,
    -(2570 as libc::c_int) as opus_int16,
    -(2760 as libc::c_int) as opus_int16,
    -(2948 as libc::c_int) as opus_int16,
    -(3136 as libc::c_int) as opus_int16,
    -(3320 as libc::c_int) as opus_int16,
    -(3502 as libc::c_int) as opus_int16,
    -(3684 as libc::c_int) as opus_int16,
    -(3862 as libc::c_int) as opus_int16,
    -(4038 as libc::c_int) as opus_int16,
    -(4212 as libc::c_int) as opus_int16,
    -(4382 as libc::c_int) as opus_int16,
    -(4552 as libc::c_int) as opus_int16,
    -(4718 as libc::c_int) as opus_int16,
    -(4880 as libc::c_int) as opus_int16,
    -(5040 as libc::c_int) as opus_int16,
    -(5198 as libc::c_int) as opus_int16,
    -(5352 as libc::c_int) as opus_int16,
    -(5502 as libc::c_int) as opus_int16,
    -(5648 as libc::c_int) as opus_int16,
    -(5792 as libc::c_int) as opus_int16,
    -(5934 as libc::c_int) as opus_int16,
    -(6070 as libc::c_int) as opus_int16,
    -(6204 as libc::c_int) as opus_int16,
    -(6332 as libc::c_int) as opus_int16,
    -(6458 as libc::c_int) as opus_int16,
    -(6580 as libc::c_int) as opus_int16,
    -(6698 as libc::c_int) as opus_int16,
    -(6812 as libc::c_int) as opus_int16,
    -(6922 as libc::c_int) as opus_int16,
    -(7026 as libc::c_int) as opus_int16,
    -(7128 as libc::c_int) as opus_int16,
    -(7226 as libc::c_int) as opus_int16,
    -(7318 as libc::c_int) as opus_int16,
    -(7406 as libc::c_int) as opus_int16,
    -(7490 as libc::c_int) as opus_int16,
    -(7568 as libc::c_int) as opus_int16,
    -(7644 as libc::c_int) as opus_int16,
    -(7714 as libc::c_int) as opus_int16,
    -(7778 as libc::c_int) as opus_int16,
    -(7840 as libc::c_int) as opus_int16,
    -(7896 as libc::c_int) as opus_int16,
    -(7946 as libc::c_int) as opus_int16,
    -(7994 as libc::c_int) as opus_int16,
    -(8034 as libc::c_int) as opus_int16,
    -(8072 as libc::c_int) as opus_int16,
    -(8104 as libc::c_int) as opus_int16,
    -(8130 as libc::c_int) as opus_int16,
    -(8152 as libc::c_int) as opus_int16,
    -(8170 as libc::c_int) as opus_int16,
    -(8182 as libc::c_int) as opus_int16,
    -(8190 as libc::c_int) as opus_int16,
    -(8192 as libc::c_int) as opus_int16,
];
