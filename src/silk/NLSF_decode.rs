use crate::silk::NLSF_stabilize::silk_NLSF_stabilize;
use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use crate::silk::structs::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "373:1"]
        pub fn silk_NLSF_unpack(
            ec_ix: *mut i16,
            pred_Q8: *mut u8,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            CB1_index: libc::c_int,
        );
    }
}
use self::main_h::silk_NLSF_unpack;
use crate::silk::structs::silk_NLSF_CB_struct;

#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn silk_NLSF_residual_dequant(
    x_Q10: *mut i16,
    indices: *const i8,
    pred_coef_Q8: *const u8,
    quant_step_size_Q16: libc::c_int,
    order: i16,
) {
    let mut i: libc::c_int = 0;
    let mut out_Q10: libc::c_int = 0;
    let mut pred_Q10: libc::c_int = 0;
    out_Q10 = 0 as libc::c_int;
    i = order as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        pred_Q10 = out_Q10 as i16 as i32 * *pred_coef_Q8.offset(i as isize) as i16 as i32
            >> 8 as libc::c_int;
        out_Q10 = ((*indices.offset(i as isize) as u32) << 10 as libc::c_int) as i32;
        if out_Q10 > 0 as libc::c_int {
            out_Q10 = out_Q10
                - (0.1f64 * ((1 as libc::c_int as i64) << 10 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32;
        } else if out_Q10 < 0 as libc::c_int {
            out_Q10 = out_Q10
                + (0.1f64 * ((1 as libc::c_int as i64) << 10 as libc::c_int) as libc::c_double
                    + 0.5f64) as i32;
        }
        out_Q10 = (pred_Q10 as libc::c_long
            + (out_Q10 as libc::c_long * quant_step_size_Q16 as i16 as i64 >> 16 as libc::c_int))
            as i32;
        *x_Q10.offset(i as isize) = out_Q10 as i16;
        i -= 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn silk_NLSF_decode(
    pNLSF_Q15: *mut i16,
    NLSFIndices: *mut i8,
    psNLSF_CB: *const silk_NLSF_CB_struct,
) {
    let mut i: libc::c_int = 0;
    let mut pred_Q8: [u8; 16] = [0; 16];
    let mut ec_ix: [i16; 16] = [0; 16];
    let mut res_Q10: [i16; 16] = [0; 16];
    let mut NLSF_Q15_tmp: i32 = 0;
    let mut pCB_element: *const u8 = 0 as *const u8;
    let mut pCB_Wght_Q9: *const i16 = 0 as *const i16;
    silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        psNLSF_CB,
        *NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int,
    );
    silk_NLSF_residual_dequant(
        res_Q10.as_mut_ptr(),
        &mut *NLSFIndices.offset(1 as libc::c_int as isize) as *mut i8 as *const i8,
        pred_Q8.as_mut_ptr() as *const u8,
        (*psNLSF_CB).quantStepSize_Q16 as libc::c_int,
        (*psNLSF_CB).order,
    );
    pCB_element = &*((*psNLSF_CB).CB1_NLSF_Q8).offset(
        (*NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int
            * (*psNLSF_CB).order as libc::c_int) as isize,
    ) as *const u8;
    pCB_Wght_Q9 = &*((*psNLSF_CB).CB1_Wght_Q9).offset(
        (*NLSFIndices.offset(0 as libc::c_int as isize) as libc::c_int
            * (*psNLSF_CB).order as libc::c_int) as isize,
    ) as *const i16;
    i = 0 as libc::c_int;
    while i < (*psNLSF_CB).order as libc::c_int {
        NLSF_Q15_tmp = ((res_Q10[i as usize] as i32 as u32) << 14 as libc::c_int) as i32
            / *pCB_Wght_Q9.offset(i as isize) as libc::c_int
            + ((*pCB_element.offset(i as isize) as i16 as u32) << 7 as libc::c_int) as i32;
        *pNLSF_Q15.offset(i as isize) = (if 0 as libc::c_int > 32767 as libc::c_int {
            if NLSF_Q15_tmp > 0 as libc::c_int {
                0 as libc::c_int
            } else if NLSF_Q15_tmp < 32767 as libc::c_int {
                32767 as libc::c_int
            } else {
                NLSF_Q15_tmp
            }
        } else if NLSF_Q15_tmp > 32767 as libc::c_int {
            32767 as libc::c_int
        } else if NLSF_Q15_tmp < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            NLSF_Q15_tmp
        }) as i16;
        i += 1;
    }
    silk_NLSF_stabilize(
        pNLSF_Q15,
        (*psNLSF_CB).deltaMin_Q15,
        (*psNLSF_CB).order as libc::c_int,
    );
}
