use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:33"]
pub mod opus_types_h {
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "184:1"]
    pub type celt_norm = libc::c_float;
    #[c2rust::src_loc = "207:9"]
    pub const EPSILON: libc::c_float = 1e-15f32;
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:33"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = opus_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: opus_uint32,
        pub end_offs: opus_uint32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: opus_uint32,
        pub rng: opus_uint32,
        pub val: opus_uint32,
        pub ext: opus_uint32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "124:1"]
    pub unsafe extern "C" fn celt_udiv(
        mut n: opus_uint32,
        mut d: opus_uint32,
    ) -> opus_uint32 {
        return n.wrapping_div(d);
    }
    use super::opus_types_h::opus_uint32;
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:33"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "62:17"]
        pub fn cos(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "143:13"]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "162:14"]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
        #[c2rust::src_loc = "165:14"]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/mathops.h:33"]
pub mod mathops_h {
    #[c2rust::src_loc = "50:9"]
    pub const cA: libc::c_float = 0.43157974f32;
    #[c2rust::src_loc = "51:9"]
    pub const cB: libc::c_float = 0.67848403f32;
    #[c2rust::src_loc = "52:9"]
    pub const cC: libc::c_float = 0.08595542f32;
    #[c2rust::src_loc = "53:9"]
    pub const cE: libc::c_float = PI / 2 as libc::c_int as libc::c_float;
    #[c2rust::src_loc = "41:9"]
    pub const PI: libc::c_float = 3.141592653f32;
    #[inline]
    #[c2rust::src_loc = "54:1"]
    pub unsafe extern "C" fn fast_atan2f(
        mut y: libc::c_float,
        mut x: libc::c_float,
    ) -> libc::c_float {
        let mut x2: libc::c_float = 0.;
        let mut y2: libc::c_float = 0.;
        x2 = x * x;
        y2 = y * y;
        if x2 + y2 < 1e-18f32 {
            return 0 as libc::c_int as libc::c_float;
        }
        if x2 < y2 {
            let mut den: libc::c_float = (y2 + cB * x2) * (y2 + cC * x2);
            return -x * y * (y2 + cA * x2) / den
                + (if y < 0 as libc::c_int as libc::c_float { -cE } else { cE });
        } else {
            let mut den_0: libc::c_float = (x2 + cB * y2) * (x2 + cC * y2);
            return x * y * (x2 + cA * y2) / den_0
                + (if y < 0 as libc::c_int as libc::c_float { -cE } else { cE })
                - (if x * y < 0 as libc::c_int as libc::c_float { -cE } else { cE });
        };
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/cwrs.h:34"]
pub mod cwrs_h {
    use super::entcode_h::{ec_dec, ec_enc};
    use super::arch_h::opus_val32;
    extern "C" {
        #[c2rust::src_loc = "46:1"]
        pub fn decode_pulses(
            _y: *mut libc::c_int,
            N: libc::c_int,
            K: libc::c_int,
            dec: *mut ec_dec,
        ) -> opus_val32;
        #[c2rust::src_loc = "44:1"]
        pub fn encode_pulses(
            _y: *const libc::c_int,
            N: libc::c_int,
            K: libc::c_int,
            enc: *mut ec_enc,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/bands.h:38"]
pub mod bands_h {
    #[c2rust::src_loc = "68:9"]
    pub const SPREAD_NONE: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/pitch.h:40"]
pub mod pitch_h {
    #[inline]
    #[c2rust::src_loc = "159:1"]
    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const opus_val16,
        mut y: *const opus_val16,
        mut N: libc::c_int,
    ) -> opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: opus_val32 = 0 as libc::c_int as opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1;
        }
        return xy;
    }
    use super::arch_h::{opus_val16, opus_val32};
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::opus_types_h::opus_uint32;
pub use self::arch_h::{opus_val16, opus_val32, celt_norm, EPSILON, celt_fatal};
pub use self::entcode_h::{ec_window, ec_ctx, ec_enc, ec_dec, celt_udiv};
use self::mathcalls_h::{cos, sqrt, fabs, floor};
pub use self::mathops_h::{cA, cB, cC, cE, PI, fast_atan2f};
use self::cwrs_h::{decode_pulses, encode_pulses};
pub use self::bands_h::SPREAD_NONE;
pub use self::pitch_h::celt_inner_prod_c;
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn exp_rotation1(
    mut X: *mut celt_norm,
    mut len: libc::c_int,
    mut stride: libc::c_int,
    mut c: opus_val16,
    mut s: opus_val16,
) {
    let mut i: libc::c_int = 0;
    let mut ms: opus_val16 = 0.;
    let mut Xptr: *mut celt_norm = 0 as *mut celt_norm;
    Xptr = X;
    ms = -s;
    i = 0 as libc::c_int;
    while i < len - stride {
        let mut x1: celt_norm = 0.;
        let mut x2: celt_norm = 0.;
        x1 = *Xptr.offset(0 as libc::c_int as isize);
        x2 = *Xptr.offset(stride as isize);
        *Xptr.offset(stride as isize) = c * x2 + s * x1;
        let fresh0 = Xptr;
        Xptr = Xptr.offset(1);
        *fresh0 = c * x1 + ms * x2;
        i += 1;
    }
    Xptr = &mut *X.offset((len - 2 as libc::c_int * stride - 1 as libc::c_int) as isize)
        as *mut celt_norm;
    i = len - 2 as libc::c_int * stride - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut x1_0: celt_norm = 0.;
        let mut x2_0: celt_norm = 0.;
        x1_0 = *Xptr.offset(0 as libc::c_int as isize);
        x2_0 = *Xptr.offset(stride as isize);
        *Xptr.offset(stride as isize) = c * x2_0 + s * x1_0;
        let fresh1 = Xptr;
        Xptr = Xptr.offset(-1);
        *fresh1 = c * x1_0 + ms * x2_0;
        i -= 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn exp_rotation(
    mut X: *mut celt_norm,
    mut len: libc::c_int,
    mut dir: libc::c_int,
    mut stride: libc::c_int,
    mut K: libc::c_int,
    mut spread: libc::c_int,
) {
    static mut SPREAD_FACTOR: [libc::c_int; 3] = [
        15 as libc::c_int,
        10 as libc::c_int,
        5 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut c: opus_val16 = 0.;
    let mut s: opus_val16 = 0.;
    let mut gain: opus_val16 = 0.;
    let mut theta: opus_val16 = 0.;
    let mut stride2: libc::c_int = 0 as libc::c_int;
    let mut factor: libc::c_int = 0;
    if 2 as libc::c_int * K >= len || spread == SPREAD_NONE {
        return;
    }
    factor = SPREAD_FACTOR[(spread - 1 as libc::c_int) as usize];
    gain = 1.0f32 * len as opus_val32 / (len + factor * K) as opus_val32;
    theta = 0.5f32 * (gain * gain);
    c = cos((0.5f32 * PI * theta) as libc::c_double) as libc::c_float;
    s = cos((0.5f32 * PI * (1.0f32 - theta)) as libc::c_double) as libc::c_float;
    if len >= 8 as libc::c_int * stride {
        stride2 = 1 as libc::c_int;
        while (stride2 * stride2 + stride2) * stride + (stride >> 2 as libc::c_int) < len
        {
            stride2 += 1;
        }
    }
    len = celt_udiv(len as opus_uint32, stride as opus_uint32) as libc::c_int;
    i = 0 as libc::c_int;
    while i < stride {
        if dir < 0 as libc::c_int {
            if stride2 != 0 {
                exp_rotation1(X.offset((i * len) as isize), len, stride2, s, c);
            }
            exp_rotation1(X.offset((i * len) as isize), len, 1 as libc::c_int, c, s);
        } else {
            exp_rotation1(X.offset((i * len) as isize), len, 1 as libc::c_int, c, -s);
            if stride2 != 0 {
                exp_rotation1(X.offset((i * len) as isize), len, stride2, s, -c);
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn normalise_residual(
    mut iy: *mut libc::c_int,
    mut X: *mut celt_norm,
    mut N: libc::c_int,
    mut Ryy: opus_val32,
    mut gain: opus_val16,
) {
    let mut i: libc::c_int = 0;
    let mut t: opus_val32 = 0.;
    let mut g: opus_val16 = 0.;
    t = Ryy;
    g = 1.0f32 / sqrt(t as libc::c_double) as libc::c_float * gain;
    i = 0 as libc::c_int;
    loop {
        *X.offset(i as isize) = g * *iy.offset(i as isize) as opus_val32;
        i += 1;
        if !(i < N) {
            break;
        }
    };
}
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn extract_collapse_mask(
    mut iy: *mut libc::c_int,
    mut N: libc::c_int,
    mut B: libc::c_int,
) -> libc::c_uint {
    let mut collapse_mask: libc::c_uint = 0;
    let mut N0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if B <= 1 as libc::c_int {
        return 1 as libc::c_int as libc::c_uint;
    }
    N0 = celt_udiv(N as opus_uint32, B as opus_uint32) as libc::c_int;
    collapse_mask = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    loop {
        let mut j: libc::c_int = 0;
        let mut tmp: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        j = 0 as libc::c_int;
        loop {
            tmp |= *iy.offset((i * N0 + j) as isize) as libc::c_uint;
            j += 1;
            if !(j < N0) {
                break;
            }
        }
        collapse_mask
            |= (((tmp != 0 as libc::c_int as libc::c_uint) as libc::c_int) << i)
                as libc::c_uint;
        i += 1;
        if !(i < B) {
            break;
        }
    }
    return collapse_mask;
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn op_pvq_search_c(
    mut X: *mut celt_norm,
    mut iy: *mut libc::c_int,
    mut K: libc::c_int,
    mut N: libc::c_int,
    mut arch: libc::c_int,
) -> opus_val16 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pulsesLeft: libc::c_int = 0;
    let mut sum: opus_val32 = 0.;
    let mut xy: opus_val32 = 0.;
    let mut yy: opus_val16 = 0.;
    let vla = N as usize;
    let mut y: Vec::<celt_norm> = ::std::vec::from_elem(0., vla);
    let vla_0 = N as usize;
    let mut signx: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    sum = 0 as libc::c_int as opus_val32;
    j = 0 as libc::c_int;
    loop {
        *signx
            .as_mut_ptr()
            .offset(
                j as isize,
            ) = (*X.offset(j as isize) < 0 as libc::c_int as libc::c_float)
            as libc::c_int;
        *X
            .offset(
                j as isize,
            ) = fabs(*X.offset(j as isize) as libc::c_double) as libc::c_float;
        *iy.offset(j as isize) = 0 as libc::c_int;
        *y.as_mut_ptr().offset(j as isize) = 0 as libc::c_int as celt_norm;
        j += 1;
        if !(j < N) {
            break;
        }
    }
    yy = 0 as libc::c_int as opus_val16;
    xy = yy;
    pulsesLeft = K;
    if K > N >> 1 as libc::c_int {
        let mut rcp: opus_val16 = 0.;
        j = 0 as libc::c_int;
        loop {
            sum += *X.offset(j as isize);
            j += 1;
            if !(j < N) {
                break;
            }
        }
        if !(sum > EPSILON && sum < 64 as libc::c_int as libc::c_float) {
            *X.offset(0 as libc::c_int as isize) = 1.0f32;
            j = 1 as libc::c_int;
            loop {
                *X.offset(j as isize) = 0 as libc::c_int as celt_norm;
                j += 1;
                if !(j < N) {
                    break;
                }
            }
            sum = 1.0f32;
        }
        rcp = (K as libc::c_float + 0.8f32) * (1.0f32 / sum);
        j = 0 as libc::c_int;
        loop {
            *iy
                .offset(
                    j as isize,
                ) = floor((rcp * *X.offset(j as isize)) as libc::c_double)
                as libc::c_int;
            *y.as_mut_ptr().offset(j as isize) = *iy.offset(j as isize) as celt_norm;
            yy = yy
                + *y.as_mut_ptr().offset(j as isize)
                    * *y.as_mut_ptr().offset(j as isize);
            xy = xy + *X.offset(j as isize) * *y.as_mut_ptr().offset(j as isize);
            let ref mut fresh2 = *y.as_mut_ptr().offset(j as isize);
            *fresh2 *= 2 as libc::c_int as libc::c_float;
            pulsesLeft -= *iy.offset(j as isize);
            j += 1;
            if !(j < N) {
                break;
            }
        }
    }
    if pulsesLeft > N + 3 as libc::c_int {
        let mut tmp: opus_val16 = pulsesLeft as opus_val16;
        yy = yy + tmp * tmp;
        yy = yy + tmp * *y.as_mut_ptr().offset(0 as libc::c_int as isize);
        *iy.offset(0 as libc::c_int as isize) += pulsesLeft;
        pulsesLeft = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < pulsesLeft {
        let mut Rxy: opus_val16 = 0.;
        let mut Ryy: opus_val16 = 0.;
        let mut best_id: libc::c_int = 0;
        let mut best_num: opus_val32 = 0.;
        let mut best_den: opus_val16 = 0.;
        best_id = 0 as libc::c_int;
        yy = yy + 1 as libc::c_int as libc::c_float;
        Rxy = xy + *X.offset(0 as libc::c_int as isize);
        Ryy = yy + *y.as_mut_ptr().offset(0 as libc::c_int as isize);
        Rxy = Rxy * Rxy;
        best_den = Ryy;
        best_num = Rxy;
        j = 1 as libc::c_int;
        loop {
            Rxy = xy + *X.offset(j as isize);
            Ryy = yy + *y.as_mut_ptr().offset(j as isize);
            Rxy = Rxy * Rxy;
            if (best_den * Rxy > Ryy * best_num) as libc::c_int as libc::c_long != 0 {
                best_den = Ryy;
                best_num = Rxy;
                best_id = j;
            }
            j += 1;
            if !(j < N) {
                break;
            }
        }
        xy = xy + *X.offset(best_id as isize);
        yy = yy + *y.as_mut_ptr().offset(best_id as isize);
        let ref mut fresh3 = *y.as_mut_ptr().offset(best_id as isize);
        *fresh3 += 2 as libc::c_int as libc::c_float;
        let ref mut fresh4 = *iy.offset(best_id as isize);
        *fresh4 += 1;
        i += 1;
    }
    j = 0 as libc::c_int;
    loop {
        *iy
            .offset(
                j as isize,
            ) = (*iy.offset(j as isize) ^ -*signx.as_mut_ptr().offset(j as isize))
            + *signx.as_mut_ptr().offset(j as isize);
        j += 1;
        if !(j < N) {
            break;
        }
    }
    return yy;
}
#[no_mangle]
#[c2rust::src_loc = "330:1"]
pub unsafe extern "C" fn alg_quant(
    mut X: *mut celt_norm,
    mut N: libc::c_int,
    mut K: libc::c_int,
    mut spread: libc::c_int,
    mut B: libc::c_int,
    mut enc: *mut ec_enc,
    mut gain: opus_val16,
    mut resynth: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_uint {
    let mut yy: opus_val16 = 0.;
    let mut collapse_mask: libc::c_uint = 0;
    if !(K > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: K>0\nalg_quant() needs at least one pulse\0" as *const u8
                as *const libc::c_char,
            b"celt/vq.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
        );
    }
    if !(N > 1 as libc::c_int) {
        celt_fatal(
            b"assertion failed: N>1\nalg_quant() needs at least two dimensions\0"
                as *const u8 as *const libc::c_char,
            b"celt/vq.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
        );
    }
    let vla = (N + 3 as libc::c_int) as usize;
    let mut iy: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    exp_rotation(X, N, 1 as libc::c_int, B, K, spread);
    yy = op_pvq_search_c(X, iy.as_mut_ptr(), K, N, arch);
    encode_pulses(iy.as_mut_ptr(), N, K, enc);
    if resynth != 0 {
        normalise_residual(iy.as_mut_ptr(), X, N, yy, gain);
        exp_rotation(X, N, -(1 as libc::c_int), B, K, spread);
    }
    collapse_mask = extract_collapse_mask(iy.as_mut_ptr(), N, B);
    return collapse_mask;
}
#[no_mangle]
#[c2rust::src_loc = "363:1"]
pub unsafe extern "C" fn alg_unquant(
    mut X: *mut celt_norm,
    mut N: libc::c_int,
    mut K: libc::c_int,
    mut spread: libc::c_int,
    mut B: libc::c_int,
    mut dec: *mut ec_dec,
    mut gain: opus_val16,
) -> libc::c_uint {
    let mut Ryy: opus_val32 = 0.;
    let mut collapse_mask: libc::c_uint = 0;
    if !(K > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: K>0\nalg_unquant() needs at least one pulse\0"
                as *const u8 as *const libc::c_char,
            b"celt/vq.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
        );
    }
    if !(N > 1 as libc::c_int) {
        celt_fatal(
            b"assertion failed: N>1\nalg_unquant() needs at least two dimensions\0"
                as *const u8 as *const libc::c_char,
            b"celt/vq.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
        );
    }
    let vla = N as usize;
    let mut iy: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    Ryy = decode_pulses(iy.as_mut_ptr(), N, K, dec);
    normalise_residual(iy.as_mut_ptr(), X, N, Ryy, gain);
    exp_rotation(X, N, -(1 as libc::c_int), B, K, spread);
    collapse_mask = extract_collapse_mask(iy.as_mut_ptr(), N, B);
    return collapse_mask;
}
#[no_mangle]
#[c2rust::src_loc = "383:1"]
pub unsafe extern "C" fn renormalise_vector(
    mut X: *mut celt_norm,
    mut N: libc::c_int,
    mut gain: opus_val16,
    mut arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut E: opus_val32 = 0.;
    let mut g: opus_val16 = 0.;
    let mut t: opus_val32 = 0.;
    let mut xptr: *mut celt_norm = 0 as *mut celt_norm;
    E = EPSILON + celt_inner_prod_c(X, X, N);
    t = E;
    g = 1.0f32 / sqrt(t as libc::c_double) as libc::c_float * gain;
    xptr = X;
    i = 0 as libc::c_int;
    while i < N {
        *xptr = g * *xptr;
        xptr = xptr.offset(1);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "410:1"]
pub unsafe extern "C" fn stereo_itheta(
    mut X: *const celt_norm,
    mut Y: *const celt_norm,
    mut stereo: libc::c_int,
    mut N: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut itheta: libc::c_int = 0;
    let mut mid: opus_val16 = 0.;
    let mut side: opus_val16 = 0.;
    let mut Emid: opus_val32 = 0.;
    let mut Eside: opus_val32 = 0.;
    Eside = EPSILON;
    Emid = Eside;
    if stereo != 0 {
        i = 0 as libc::c_int;
        while i < N {
            let mut m: celt_norm = 0.;
            let mut s: celt_norm = 0.;
            m = *X.offset(i as isize) + *Y.offset(i as isize);
            s = *X.offset(i as isize) - *Y.offset(i as isize);
            Emid = Emid + m * m;
            Eside = Eside + s * s;
            i += 1;
        }
    } else {
        Emid += celt_inner_prod_c(X, X, N);
        Eside += celt_inner_prod_c(Y, Y, N);
    }
    mid = sqrt(Emid as libc::c_double) as libc::c_float;
    side = sqrt(Eside as libc::c_double) as libc::c_float;
    itheta = floor(
        (0.5f32
            + 16384 as libc::c_int as libc::c_float * 0.63662f32
                * fast_atan2f(side, mid)) as libc::c_double,
    ) as libc::c_int;
    return itheta;
}
