use ::libc;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "286:1"]
        pub fn silk_NLSF2A(a_Q12: *mut i16, NLSF: *const i16, d: libc::c_int, arch: libc::c_int);
        #[c2rust::src_loc = "329:1"]
        pub fn silk_NLSF_VQ_weights_laroia(
            pNLSFW_Q_OUT: *mut i16,
            pNLSF_Q15: *const i16,
            D: libc::c_int,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:32"]
pub mod main_h {
    use crate::silk::structs::silk_NLSF_CB_struct;
    extern "C" {
        #[c2rust::src_loc = "202:1"]
        pub fn silk_interpolate(
            xi: *mut i16,
            x0: *const i16,
            x1: *const i16,
            ifact_Q2: libc::c_int,
            d: libc::c_int,
        );
        #[c2rust::src_loc = "338:1"]
        pub fn silk_NLSF_encode(
            NLSFIndices: *mut i8,
            pNLSF_Q15: *mut i16,
            psNLSF_CB: *const silk_NLSF_CB_struct,
            pW_QW: *const i16,
            NLSF_mu_Q20: libc::c_int,
            nSurvivors: libc::c_int,
            signalType: libc::c_int,
        ) -> i32;
    }
}
use self::main_h::{silk_NLSF_encode, silk_interpolate};
use crate::celt::celt::celt_fatal;

use crate::silk::structs::silk_encoder_state;

use self::SigProc_FIX_h::{silk_NLSF2A, silk_NLSF_VQ_weights_laroia};
use crate::externs::memcpy;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_process_NLSFs(
    psEncC: *mut silk_encoder_state,
    PredCoef_Q12: *mut [i16; 16],
    pNLSF_Q15: *mut i16,
    prev_NLSFq_Q15: *const i16,
) {
    let mut i: libc::c_int = 0;
    let mut doInterpolate: libc::c_int = 0;
    let mut NLSF_mu_Q20: libc::c_int = 0;
    let mut i_sqr_Q15: i16 = 0;
    let mut pNLSF0_temp_Q15: [i16; 16] = [0; 16];
    let mut pNLSFW_QW: [i16; 16] = [0; 16];
    let mut pNLSFW0_temp_QW: [i16; 16] = [0; 16];
    if !((*psEncC).useInterpolatedNLSFs == 1 as libc::c_int
        || (*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int
            == (1 as libc::c_int) << 2 as libc::c_int)
    {
        celt_fatal(
            b"assertion failed: psEncC->useInterpolatedNLSFs == 1 || psEncC->indices.NLSFInterpCoef_Q2 == ( 1 << 2 )\0"
                as *const u8 as *const libc::c_char,
            b"silk/process_NLSFs.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
        );
    }
    NLSF_mu_Q20 = ((0.003f64 * ((1 as libc::c_int as i64) << 20 as libc::c_int) as libc::c_double
        + 0.5f64) as i32 as libc::c_long
        + ((-0.001f64 * ((1 as libc::c_int as i64) << 28 as libc::c_int) as libc::c_double + 0.5f64)
            as i32 as libc::c_long
            * (*psEncC).speech_activity_Q8 as i16 as i64
            >> 16 as libc::c_int)) as i32;
    if (*psEncC).nb_subfr == 2 as libc::c_int {
        NLSF_mu_Q20 = NLSF_mu_Q20 + (NLSF_mu_Q20 >> 1 as libc::c_int);
    }
    if !(NLSF_mu_Q20 > 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: NLSF_mu_Q20 > 0\0" as *const u8 as *const libc::c_char,
            b"silk/process_NLSFs.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
        );
    }
    silk_NLSF_VQ_weights_laroia(
        pNLSFW_QW.as_mut_ptr(),
        pNLSF_Q15 as *const i16,
        (*psEncC).predictLPCOrder,
    );
    doInterpolate = ((*psEncC).useInterpolatedNLSFs == 1 as libc::c_int
        && ((*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int) < 4 as libc::c_int)
        as libc::c_int;
    if doInterpolate != 0 {
        silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            prev_NLSFq_Q15,
            pNLSF_Q15 as *const i16,
            (*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int,
            (*psEncC).predictLPCOrder,
        );
        silk_NLSF_VQ_weights_laroia(
            pNLSFW0_temp_QW.as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
        );
        i_sqr_Q15 = ((((*psEncC).indices.NLSFInterpCoef_Q2 as i16 as i32
            * (*psEncC).indices.NLSFInterpCoef_Q2 as i16 as i32) as u32)
            << 11 as libc::c_int) as i32 as i16;
        i = 0 as libc::c_int;
        while i < (*psEncC).predictLPCOrder {
            pNLSFW_QW[i as usize] = ((pNLSFW_QW[i as usize] as libc::c_int >> 1 as libc::c_int)
                + (pNLSFW0_temp_QW[i as usize] as i32 * i_sqr_Q15 as i32 >> 16 as libc::c_int))
                as i16;
            i += 1;
        }
    }
    silk_NLSF_encode(
        ((*psEncC).indices.NLSFIndices).as_mut_ptr(),
        pNLSF_Q15,
        (*psEncC).psNLSF_CB,
        pNLSFW_QW.as_mut_ptr(),
        NLSF_mu_Q20,
        (*psEncC).NLSF_MSVQ_Survivors,
        (*psEncC).indices.signalType as libc::c_int,
    );
    silk_NLSF2A(
        (*PredCoef_Q12.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        pNLSF_Q15 as *const i16,
        (*psEncC).predictLPCOrder,
        (*psEncC).arch,
    );
    if doInterpolate != 0 {
        silk_interpolate(
            pNLSF0_temp_Q15.as_mut_ptr(),
            prev_NLSFq_Q15,
            pNLSF_Q15 as *const i16,
            (*psEncC).indices.NLSFInterpCoef_Q2 as libc::c_int,
            (*psEncC).predictLPCOrder,
        );
        silk_NLSF2A(
            (*PredCoef_Q12.offset(0 as libc::c_int as isize)).as_mut_ptr(),
            pNLSF0_temp_Q15.as_mut_ptr(),
            (*psEncC).predictLPCOrder,
            (*psEncC).arch,
        );
    } else {
        if !((*psEncC).predictLPCOrder <= 16 as libc::c_int) {
            celt_fatal(
                b"assertion failed: psEncC->predictLPCOrder <= MAX_LPC_ORDER\0" as *const u8
                    as *const libc::c_char,
                b"silk/process_NLSFs.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int,
            );
        }
        memcpy(
            (*PredCoef_Q12.offset(0 as libc::c_int as isize)).as_mut_ptr() as *mut libc::c_void,
            (*PredCoef_Q12.offset(1 as libc::c_int as isize)).as_mut_ptr() as *const libc::c_void,
            ((*psEncC).predictLPCOrder as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    };
}
