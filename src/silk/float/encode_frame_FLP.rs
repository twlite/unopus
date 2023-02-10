use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = libc::c_schar;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:33"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:33"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int32, opus_int16};
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
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
        return (*_this).nbits_total - (EC_CLZ0 - ((*_this).rng).leading_zeros() as i32);
    }
    use super::opus_types_h::opus_uint32;
    use super::ecintrin_h::EC_CLZ0;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:33"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct silk_nsq_state {
        pub xq: [opus_int16; 640],
        pub sLTP_shp_Q14: [opus_int32; 640],
        pub sLPC_Q14: [opus_int32; 96],
        pub sAR2_Q14: [opus_int32; 24],
        pub sLF_AR_shp_Q14: opus_int32,
        pub sDiff_shp_Q14: opus_int32,
        pub lagPrev: libc::c_int,
        pub sLTP_buf_idx: libc::c_int,
        pub sLTP_shp_buf_idx: libc::c_int,
        pub rand_seed: opus_int32,
        pub prev_gain_Q16: opus_int32,
        pub rewhite_flag: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:9"]
    pub struct silk_VAD_state {
        pub AnaState: [opus_int32; 2],
        pub AnaState1: [opus_int32; 2],
        pub AnaState2: [opus_int32; 2],
        pub XnrgSubfr: [opus_int32; 4],
        pub NrgRatioSmth_Q8: [opus_int32; 4],
        pub HPstate: opus_int16,
        pub NL: [opus_int32; 4],
        pub inv_NL: [opus_int32; 4],
        pub NoiseLevelBias: [opus_int32; 4],
        pub counter: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:9"]
    pub struct silk_LP_state {
        pub In_LP_State: [opus_int32; 2],
        pub transition_frame_no: opus_int32,
        pub mode: libc::c_int,
        pub saved_fs_kHz: opus_int32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:9"]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: opus_int16,
        pub order: opus_int16,
        pub quantStepSize_Q16: opus_int16,
        pub invQuantStepSize_Q6: opus_int16,
        pub CB1_NLSF_Q8: *const opus_uint8,
        pub CB1_Wght_Q9: *const opus_int16,
        pub CB1_iCDF: *const opus_uint8,
        pub pred_Q8: *const opus_uint8,
        pub ec_sel: *const opus_uint8,
        pub ec_iCDF: *const opus_uint8,
        pub ec_Rates_Q5: *const opus_uint8,
        pub deltaMin_Q15: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct SideInfoIndices {
        pub GainsIndices: [opus_int8; 4],
        pub LTPIndex: [opus_int8; 4],
        pub NLSFIndices: [opus_int8; 17],
        pub lagIndex: opus_int16,
        pub contourIndex: opus_int8,
        pub signalType: opus_int8,
        pub quantOffsetType: opus_int8,
        pub NLSFInterpCoef_Q2: opus_int8,
        pub PERIndex: opus_int8,
        pub LTP_scaleIndex: opus_int8,
        pub Seed: opus_int8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct silk_encoder_state {
        pub In_HP_State: [opus_int32; 2],
        pub variable_HP_smth1_Q15: opus_int32,
        pub variable_HP_smth2_Q15: opus_int32,
        pub sLP: silk_LP_state,
        pub sVAD: silk_VAD_state,
        pub sNSQ: silk_nsq_state,
        pub prev_NLSFq_Q15: [opus_int16; 16],
        pub speech_activity_Q8: libc::c_int,
        pub allow_bandwidth_switch: libc::c_int,
        pub LBRRprevLastGainIndex: opus_int8,
        pub prevSignalType: opus_int8,
        pub prevLag: libc::c_int,
        pub pitch_LPC_win_length: libc::c_int,
        pub max_pitch_lag: libc::c_int,
        pub API_fs_Hz: opus_int32,
        pub prev_API_fs_Hz: opus_int32,
        pub maxInternal_fs_Hz: libc::c_int,
        pub minInternal_fs_Hz: libc::c_int,
        pub desiredInternal_fs_Hz: libc::c_int,
        pub fs_kHz: libc::c_int,
        pub nb_subfr: libc::c_int,
        pub frame_length: libc::c_int,
        pub subfr_length: libc::c_int,
        pub ltp_mem_length: libc::c_int,
        pub la_pitch: libc::c_int,
        pub la_shape: libc::c_int,
        pub shapeWinLength: libc::c_int,
        pub TargetRate_bps: opus_int32,
        pub PacketSize_ms: libc::c_int,
        pub PacketLoss_perc: libc::c_int,
        pub frameCounter: opus_int32,
        pub Complexity: libc::c_int,
        pub nStatesDelayedDecision: libc::c_int,
        pub useInterpolatedNLSFs: libc::c_int,
        pub shapingLPCOrder: libc::c_int,
        pub predictLPCOrder: libc::c_int,
        pub pitchEstimationComplexity: libc::c_int,
        pub pitchEstimationLPCOrder: libc::c_int,
        pub pitchEstimationThreshold_Q16: opus_int32,
        pub sum_log_gain_Q7: opus_int32,
        pub NLSF_MSVQ_Survivors: libc::c_int,
        pub first_frame_after_reset: libc::c_int,
        pub controlled_since_last_payload: libc::c_int,
        pub warping_Q16: libc::c_int,
        pub useCBR: libc::c_int,
        pub prefillFlag: libc::c_int,
        pub pitch_lag_low_bits_iCDF: *const opus_uint8,
        pub pitch_contour_iCDF: *const opus_uint8,
        pub psNLSF_CB: *const silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [libc::c_int; 4],
        pub input_tilt_Q15: libc::c_int,
        pub SNR_dB_Q7: libc::c_int,
        pub VAD_flags: [opus_int8; 3],
        pub LBRR_flag: opus_int8,
        pub LBRR_flags: [libc::c_int; 3],
        pub indices: SideInfoIndices,
        pub pulses: [opus_int8; 320],
        pub arch: libc::c_int,
        pub inputBuf: [opus_int16; 322],
        pub inputBufIx: libc::c_int,
        pub nFramesPerPacket: libc::c_int,
        pub nFramesEncoded: libc::c_int,
        pub nChannelsAPI: libc::c_int,
        pub nChannelsInternal: libc::c_int,
        pub channelNb: libc::c_int,
        pub frames_since_onset: libc::c_int,
        pub ec_prevSignalType: libc::c_int,
        pub ec_prevLagIndex: opus_int16,
        pub resampler_state: silk_resampler_state_struct,
        pub useDTX: libc::c_int,
        pub inDTX: libc::c_int,
        pub noSpeechCounter: libc::c_int,
        pub useInBandFEC: libc::c_int,
        pub LBRR_enabled: libc::c_int,
        pub LBRR_GainIncreases: libc::c_int,
        pub indices_LBRR: [SideInfoIndices; 3],
        pub pulses_LBRR: [[opus_int8; 320]; 3],
    }
    use super::opus_types_h::{opus_int16, opus_int32, opus_uint8, opus_int8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/structs_FLP.h:33"]
pub mod structs_FLP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: opus_int8,
        pub HarmShapeGain_smth: libc::c_float,
        pub Tilt_smth: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct silk_encoder_state_FLP {
        pub sCmn: silk_encoder_state,
        pub sShape: silk_shape_state_FLP,
        pub x_buf: [libc::c_float; 720],
        pub LTPCorr: libc::c_float,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub struct silk_encoder_control_FLP {
        pub Gains: [libc::c_float; 4],
        pub PredCoef: [[libc::c_float; 16]; 2],
        pub LTPCoef: [libc::c_float; 20],
        pub LTP_scale: libc::c_float,
        pub pitchL: [libc::c_int; 4],
        pub AR: [libc::c_float; 96],
        pub LF_MA_shp: [libc::c_float; 4],
        pub LF_AR_shp: [libc::c_float; 4],
        pub Tilt: [libc::c_float; 4],
        pub HarmShapeGain: [libc::c_float; 4],
        pub Lambda: libc::c_float,
        pub input_quality: libc::c_float,
        pub coding_quality: libc::c_float,
        pub predGain: libc::c_float,
        pub LTPredCodGain: libc::c_float,
        pub ResNrg: [libc::c_float; 4],
        pub GainsUnq_Q16: [opus_int32; 4],
        pub lastGainIndexPrev: opus_int8,
    }
    use super::opus_types_h::{opus_int8, opus_int32};
    use super::structs_h::silk_encoder_state;
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "861:12"]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:33"]
pub mod SigProc_FIX_h {
    #[inline]
    #[c2rust::src_loc = "546:1"]
    pub unsafe extern "C" fn silk_min_int(
        mut a: libc::c_int,
        mut b: libc::c_int,
    ) -> libc::c_int {
        return if a < b { a } else { b };
    }
    use super::opus_types_h::opus_int32;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn silk_log2lin(inLog_Q7: opus_int32) -> opus_int32;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:33"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:33"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int = ::core::mem::size_of::<libc::c_uint>()
        as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/define.h:33"]
pub mod define_h {
    #[c2rust::src_loc = "112:9"]
    pub const LA_SHAPE_MS: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "123:9"]
    pub const N_LEVELS_QGAIN: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "71:9"]
    pub const TYPE_UNVOICED: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "56:9"]
    pub const NB_SPEECH_FRAMES_BEFORE_DTX: libc::c_int = 10 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const MAX_CONSECUTIVE_DTX: libc::c_int = 20 as libc::c_int;
    #[c2rust::src_loc = "70:9"]
    pub const TYPE_NO_VOICE_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "62:9"]
    pub const VAD_NO_ACTIVITY: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "77:9"]
    pub const CODE_CONDITIONALLY: libc::c_int = 2 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/main_FLP.h:33"]
pub mod main_FLP_h {
    use super::structs_FLP_h::{silk_encoder_state_FLP, silk_encoder_control_FLP};
    use super::structs_h::{SideInfoIndices, silk_nsq_state};
    use super::opus_types_h::opus_int8;
    extern "C" {
        #[c2rust::src_loc = "119:1"]
        pub fn silk_find_pitch_lags_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            res: *mut libc::c_float,
            x: *const libc::c_float,
            arch: libc::c_int,
        );
        #[c2rust::src_loc = "92:1"]
        pub fn silk_noise_shape_analysis_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            pitch_res: *const libc::c_float,
            x: *const libc::c_float,
        );
        #[c2rust::src_loc = "128:1"]
        pub fn silk_find_pred_coefs_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            res_pitch: *const libc::c_float,
            x: *const libc::c_float,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "210:1"]
        pub fn silk_process_gains_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            condCoding: libc::c_int,
        );
        #[c2rust::src_loc = "273:1"]
        pub fn silk_NSQ_wrapper_FLP(
            psEnc: *mut silk_encoder_state_FLP,
            psEncCtrl: *mut silk_encoder_control_FLP,
            psIndices: *mut SideInfoIndices,
            psNSQ: *mut silk_nsq_state,
            pulses: *mut opus_int8,
            x: *const libc::c_float,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:33"]
pub mod SigProc_FLP_h {
    #[inline]
    #[c2rust::src_loc = "175:1"]
    pub unsafe extern "C" fn silk_short2float_array(
        mut out: *mut libc::c_float,
        mut in_0: *const opus_int16,
        mut length: opus_int32,
    ) {
        let mut k: opus_int32 = 0;
        k = length - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            *out.offset(k as isize) = *in_0.offset(k as isize) as libc::c_float;
            k -= 1;
        }
    }
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/main.h:33"]
pub mod main_h {
    use super::entcode_h::ec_enc;
    use super::opus_types_h::{opus_int8, opus_int32, opus_int16};
    use super::structs_h::{silk_encoder_state, silk_LP_state};
    extern "C" {
        #[c2rust::src_loc = "156:1"]
        pub fn silk_encode_pulses(
            psRangeEnc: *mut ec_enc,
            signalType: libc::c_int,
            quantOffsetType: libc::c_int,
            pulses: *mut opus_int8,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "178:1"]
        pub fn silk_gains_quant(
            ind: *mut opus_int8,
            gain_Q16: *mut opus_int32,
            prev_ind: *mut opus_int8,
            conditional: libc::c_int,
            nb_subfr: libc::c_int,
        );
        #[c2rust::src_loc = "187:1"]
        pub fn silk_gains_dequant(
            gain_Q16: *mut opus_int32,
            ind: *const opus_int8,
            prev_ind: *mut opus_int8,
            conditional: libc::c_int,
            nb_subfr: libc::c_int,
        );
        #[c2rust::src_loc = "196:1"]
        pub fn silk_gains_ID(ind: *const opus_int8, nb_subfr: libc::c_int) -> opus_int32;
        #[c2rust::src_loc = "309:1"]
        pub fn silk_VAD_GetSA_Q8_c(
            psEncC: *mut silk_encoder_state,
            pIn: *const opus_int16,
        ) -> libc::c_int;
        #[c2rust::src_loc = "321:1"]
        pub fn silk_LP_variable_cutoff(
            psLP: *mut silk_LP_state,
            frame: *mut opus_int16,
            frame_length: libc::c_int,
        );
        #[c2rust::src_loc = "468:1"]
        pub fn silk_encode_indices(
            psEncC: *mut silk_encoder_state,
            psRangeEnc: *mut ec_enc,
            FrameIndex: libc::c_int,
            encode_LBRR: libc::c_int,
            condCoding: libc::c_int,
        );
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
pub use self::types_h::{
    __int8_t, __uint8_t, __int16_t, __int32_t, __uint32_t, __int64_t,
};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::opus_types_h::{
    opus_int8, opus_uint8, opus_int16, opus_int32, opus_uint32, opus_int64,
};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, C2RustUnnamed, silk_resampler_state_struct,
};
pub use self::entcode_h::{ec_window, ec_ctx, ec_enc, ec_tell};
pub use self::structs_h::{
    silk_nsq_state, silk_VAD_state, silk_LP_state, silk_NLSF_CB_struct, SideInfoIndices,
    silk_encoder_state,
};
pub use self::structs_FLP_h::{
    silk_shape_state_FLP, silk_encoder_state_FLP, silk_encoder_control_FLP,
};
use self::stdlib_h::abs;
pub use self::SigProc_FIX_h::{silk_min_int, silk_log2lin};
use self::arch_h::celt_fatal;
pub use self::limits_h::CHAR_BIT;
pub use self::ecintrin_h::EC_CLZ0;
use self::string_h::{memmove, memcpy};
pub use self::define_h::{
    LA_SHAPE_MS, N_LEVELS_QGAIN, TYPE_UNVOICED, NB_SPEECH_FRAMES_BEFORE_DTX,
    MAX_CONSECUTIVE_DTX, TYPE_NO_VOICE_ACTIVITY, VAD_NO_ACTIVITY, CODE_CONDITIONALLY,
};
use self::main_FLP_h::{
    silk_find_pitch_lags_FLP, silk_noise_shape_analysis_FLP, silk_find_pred_coefs_FLP,
    silk_process_gains_FLP, silk_NSQ_wrapper_FLP,
};
pub use self::SigProc_FLP_h::silk_short2float_array;
use self::main_h::{
    silk_encode_pulses, silk_gains_quant, silk_gains_dequant, silk_gains_ID,
    silk_VAD_GetSA_Q8_c, silk_LP_variable_cutoff, silk_encode_indices,
};
pub use self::internal::__CHAR_BIT__;
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn silk_encode_do_VAD_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut activity: libc::c_int,
) {
    let activity_threshold: libc::c_int = ((0.05f32
        * ((1 as libc::c_int as opus_int64) << 8 as libc::c_int) as libc::c_float)
        as libc::c_double + 0.5f64) as opus_int32;
    silk_VAD_GetSA_Q8_c(
        &mut (*psEnc).sCmn,
        ((*psEnc).sCmn.inputBuf).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *const opus_int16,
    );
    if activity == VAD_NO_ACTIVITY
        && (*psEnc).sCmn.speech_activity_Q8 >= activity_threshold
    {
        (*psEnc).sCmn.speech_activity_Q8 = activity_threshold - 1 as libc::c_int;
    }
    if (*psEnc).sCmn.speech_activity_Q8 < activity_threshold {
        (*psEnc).sCmn.indices.signalType = TYPE_NO_VOICE_ACTIVITY as opus_int8;
        (*psEnc).sCmn.noSpeechCounter += 1;
        if (*psEnc).sCmn.noSpeechCounter <= NB_SPEECH_FRAMES_BEFORE_DTX {
            (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        } else if (*psEnc).sCmn.noSpeechCounter
            > MAX_CONSECUTIVE_DTX + NB_SPEECH_FRAMES_BEFORE_DTX
        {
            (*psEnc).sCmn.noSpeechCounter = NB_SPEECH_FRAMES_BEFORE_DTX;
            (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        }
        (*psEnc)
            .sCmn
            .VAD_flags[(*psEnc).sCmn.nFramesEncoded
            as usize] = 0 as libc::c_int as opus_int8;
    } else {
        (*psEnc).sCmn.noSpeechCounter = 0 as libc::c_int;
        (*psEnc).sCmn.inDTX = 0 as libc::c_int;
        (*psEnc).sCmn.indices.signalType = TYPE_UNVOICED as opus_int8;
        (*psEnc)
            .sCmn
            .VAD_flags[(*psEnc).sCmn.nFramesEncoded
            as usize] = 1 as libc::c_int as opus_int8;
    };
}
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn silk_encode_frame_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut pnBytesOut: *mut opus_int32,
    mut psRangeEnc: *mut ec_enc,
    mut condCoding: libc::c_int,
    mut maxBits: libc::c_int,
    mut useCBR: libc::c_int,
) -> libc::c_int {
    let mut sEncCtrl: silk_encoder_control_FLP = silk_encoder_control_FLP {
        Gains: [0.; 4],
        PredCoef: [[0.; 16]; 2],
        LTPCoef: [0.; 20],
        LTP_scale: 0.,
        pitchL: [0; 4],
        AR: [0.; 96],
        LF_MA_shp: [0.; 4],
        LF_AR_shp: [0.; 4],
        Tilt: [0.; 4],
        HarmShapeGain: [0.; 4],
        Lambda: 0.,
        input_quality: 0.,
        coding_quality: 0.,
        predGain: 0.,
        LTPredCodGain: 0.,
        ResNrg: [0.; 4],
        GainsUnq_Q16: [0; 4],
        lastGainIndexPrev: 0,
    };
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut maxIter: libc::c_int = 0;
    let mut found_upper: libc::c_int = 0;
    let mut found_lower: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut x_frame: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut res_pitch_frame: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut res_pitch: [libc::c_float; 672] = [0.; 672];
    let mut sRangeEnc_copy: ec_enc = ec_enc {
        buf: 0 as *mut libc::c_uchar,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut sRangeEnc_copy2: ec_enc = ec_enc {
        buf: 0 as *mut libc::c_uchar,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut sNSQ_copy: silk_nsq_state = silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    let mut sNSQ_copy2: silk_nsq_state = silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    let mut seed_copy: opus_int32 = 0;
    let mut nBits: opus_int32 = 0;
    let mut nBits_lower: opus_int32 = 0;
    let mut nBits_upper: opus_int32 = 0;
    let mut gainMult_lower: opus_int32 = 0;
    let mut gainMult_upper: opus_int32 = 0;
    let mut gainsID: opus_int32 = 0;
    let mut gainsID_lower: opus_int32 = 0;
    let mut gainsID_upper: opus_int32 = 0;
    let mut gainMult_Q8: opus_int16 = 0;
    let mut ec_prevLagIndex_copy: opus_int16 = 0;
    let mut ec_prevSignalType_copy: libc::c_int = 0;
    let mut LastGainIndex_copy2: opus_int8 = 0;
    let mut pGains_Q16: [opus_int32; 4] = [0; 4];
    let mut ec_buf_copy: [opus_uint8; 1275] = [0; 1275];
    let mut gain_lock: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    let mut best_gain_mult: [opus_int16; 4] = [0; 4];
    let mut best_sum: [libc::c_int; 4] = [0; 4];
    gainMult_upper = 0 as libc::c_int;
    gainMult_lower = gainMult_upper;
    nBits_upper = gainMult_lower;
    nBits_lower = nBits_upper;
    LastGainIndex_copy2 = nBits_lower as opus_int8;
    let fresh0 = (*psEnc).sCmn.frameCounter;
    (*psEnc).sCmn.frameCounter = (*psEnc).sCmn.frameCounter + 1;
    (*psEnc).sCmn.indices.Seed = (fresh0 & 3 as libc::c_int) as opus_int8;
    x_frame = ((*psEnc).x_buf)
        .as_mut_ptr()
        .offset((*psEnc).sCmn.ltp_mem_length as isize);
    res_pitch_frame = res_pitch
        .as_mut_ptr()
        .offset((*psEnc).sCmn.ltp_mem_length as isize);
    silk_LP_variable_cutoff(
        &mut (*psEnc).sCmn.sLP,
        ((*psEnc).sCmn.inputBuf).as_mut_ptr().offset(1 as libc::c_int as isize),
        (*psEnc).sCmn.frame_length,
    );
    silk_short2float_array(
        x_frame.offset((LA_SHAPE_MS * (*psEnc).sCmn.fs_kHz) as isize),
        ((*psEnc).sCmn.inputBuf).as_mut_ptr().offset(1 as libc::c_int as isize),
        (*psEnc).sCmn.frame_length,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *x_frame
            .offset(
                (LA_SHAPE_MS * (*psEnc).sCmn.fs_kHz
                    + i * ((*psEnc).sCmn.frame_length >> 3 as libc::c_int)) as isize,
            ) += (1 as libc::c_int - (i & 2 as libc::c_int)) as libc::c_float * 1e-6f32;
        i += 1;
    }
    if (*psEnc).sCmn.prefillFlag == 0 {
        silk_find_pitch_lags_FLP(
            psEnc,
            &mut sEncCtrl,
            res_pitch.as_mut_ptr(),
            x_frame as *const libc::c_float,
            (*psEnc).sCmn.arch,
        );
        silk_noise_shape_analysis_FLP(psEnc, &mut sEncCtrl, res_pitch_frame, x_frame);
        silk_find_pred_coefs_FLP(
            psEnc,
            &mut sEncCtrl,
            res_pitch_frame as *const libc::c_float,
            x_frame as *const libc::c_float,
            condCoding,
        );
        silk_process_gains_FLP(psEnc, &mut sEncCtrl, condCoding);
        silk_LBRR_encode_FLP(
            psEnc,
            &mut sEncCtrl,
            x_frame as *const libc::c_float,
            condCoding,
        );
        maxIter = 6 as libc::c_int;
        gainMult_Q8 = ((1 as libc::c_int as libc::c_long
            * ((1 as libc::c_int as opus_int64) << 8 as libc::c_int)) as libc::c_double
            + 0.5f64) as opus_int32 as opus_int16;
        found_lower = 0 as libc::c_int;
        found_upper = 0 as libc::c_int;
        gainsID = silk_gains_ID(
            ((*psEnc).sCmn.indices.GainsIndices).as_mut_ptr() as *const opus_int8,
            (*psEnc).sCmn.nb_subfr,
        );
        gainsID_lower = -(1 as libc::c_int);
        gainsID_upper = -(1 as libc::c_int);
        memcpy(
            &mut sRangeEnc_copy as *mut ec_enc as *mut libc::c_void,
            psRangeEnc as *const libc::c_void,
            ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
        );
        memcpy(
            &mut sNSQ_copy as *mut silk_nsq_state as *mut libc::c_void,
            &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state as *const libc::c_void,
            ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
        );
        seed_copy = (*psEnc).sCmn.indices.Seed as opus_int32;
        ec_prevLagIndex_copy = (*psEnc).sCmn.ec_prevLagIndex;
        ec_prevSignalType_copy = (*psEnc).sCmn.ec_prevSignalType;
        iter = 0 as libc::c_int;
        loop {
            if gainsID == gainsID_lower {
                nBits = nBits_lower;
            } else if gainsID == gainsID_upper {
                nBits = nBits_upper;
            } else {
                if iter > 0 as libc::c_int {
                    memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy as *mut ec_enc as *const libc::c_void,
                        ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
                    );
                    memcpy(
                        &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state
                            as *mut libc::c_void,
                        &mut sNSQ_copy as *mut silk_nsq_state as *const libc::c_void,
                        ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
                    );
                    (*psEnc).sCmn.indices.Seed = seed_copy as opus_int8;
                    (*psEnc).sCmn.ec_prevLagIndex = ec_prevLagIndex_copy;
                    (*psEnc).sCmn.ec_prevSignalType = ec_prevSignalType_copy;
                }
                silk_NSQ_wrapper_FLP(
                    psEnc,
                    &mut sEncCtrl,
                    &mut (*psEnc).sCmn.indices,
                    &mut (*psEnc).sCmn.sNSQ,
                    ((*psEnc).sCmn.pulses).as_mut_ptr(),
                    x_frame as *const libc::c_float,
                );
                if iter == maxIter && found_lower == 0 {
                    memcpy(
                        &mut sRangeEnc_copy2 as *mut ec_enc as *mut libc::c_void,
                        psRangeEnc as *const libc::c_void,
                        ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
                    );
                }
                silk_encode_indices(
                    &mut (*psEnc).sCmn,
                    psRangeEnc,
                    (*psEnc).sCmn.nFramesEncoded,
                    0 as libc::c_int,
                    condCoding,
                );
                silk_encode_pulses(
                    psRangeEnc,
                    (*psEnc).sCmn.indices.signalType as libc::c_int,
                    (*psEnc).sCmn.indices.quantOffsetType as libc::c_int,
                    ((*psEnc).sCmn.pulses).as_mut_ptr(),
                    (*psEnc).sCmn.frame_length,
                );
                nBits = ec_tell(psRangeEnc);
                if iter == maxIter && found_lower == 0 && nBits > maxBits {
                    memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy2 as *mut ec_enc as *const libc::c_void,
                        ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
                    );
                    (*psEnc).sShape.LastGainIndex = sEncCtrl.lastGainIndexPrev;
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.nb_subfr {
                        (*psEnc)
                            .sCmn
                            .indices
                            .GainsIndices[i as usize] = 4 as libc::c_int as opus_int8;
                        i += 1;
                    }
                    if condCoding != CODE_CONDITIONALLY {
                        (*psEnc)
                            .sCmn
                            .indices
                            .GainsIndices[0 as libc::c_int
                            as usize] = sEncCtrl.lastGainIndexPrev;
                    }
                    (*psEnc).sCmn.ec_prevLagIndex = ec_prevLagIndex_copy;
                    (*psEnc).sCmn.ec_prevSignalType = ec_prevSignalType_copy;
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.frame_length {
                        (*psEnc).sCmn.pulses[i as usize] = 0 as libc::c_int as opus_int8;
                        i += 1;
                    }
                    silk_encode_indices(
                        &mut (*psEnc).sCmn,
                        psRangeEnc,
                        (*psEnc).sCmn.nFramesEncoded,
                        0 as libc::c_int,
                        condCoding,
                    );
                    silk_encode_pulses(
                        psRangeEnc,
                        (*psEnc).sCmn.indices.signalType as libc::c_int,
                        (*psEnc).sCmn.indices.quantOffsetType as libc::c_int,
                        ((*psEnc).sCmn.pulses).as_mut_ptr(),
                        (*psEnc).sCmn.frame_length,
                    );
                    nBits = ec_tell(psRangeEnc);
                }
                if useCBR == 0 as libc::c_int && iter == 0 as libc::c_int
                    && nBits <= maxBits
                {
                    break;
                }
            }
            if iter == maxIter {
                if found_lower != 0 && (gainsID == gainsID_lower || nBits > maxBits) {
                    memcpy(
                        psRangeEnc as *mut libc::c_void,
                        &mut sRangeEnc_copy2 as *mut ec_enc as *const libc::c_void,
                        ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
                    );
                    if !(sRangeEnc_copy2.offs <= 1275 as libc::c_int as libc::c_uint) {
                        celt_fatal(
                            b"assertion failed: sRangeEnc_copy2.offs <= 1275\0"
                                as *const u8 as *const libc::c_char,
                            b"silk/float/encode_frame_FLP.c\0" as *const u8
                                as *const libc::c_char,
                            251 as libc::c_int,
                        );
                    }
                    memcpy(
                        (*psRangeEnc).buf as *mut libc::c_void,
                        ec_buf_copy.as_mut_ptr() as *const libc::c_void,
                        sRangeEnc_copy2.offs as libc::c_ulong,
                    );
                    memcpy(
                        &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state
                            as *mut libc::c_void,
                        &mut sNSQ_copy2 as *mut silk_nsq_state as *const libc::c_void,
                        ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
                    );
                    (*psEnc).sShape.LastGainIndex = LastGainIndex_copy2;
                }
                break;
            } else {
                if nBits > maxBits {
                    if found_lower == 0 as libc::c_int && iter >= 2 as libc::c_int {
                        sEncCtrl
                            .Lambda = if sEncCtrl.Lambda * 1.5f32 > 1.5f32 {
                            sEncCtrl.Lambda * 1.5f32
                        } else {
                            1.5f32
                        };
                        (*psEnc)
                            .sCmn
                            .indices
                            .quantOffsetType = 0 as libc::c_int as opus_int8;
                        found_upper = 0 as libc::c_int;
                        gainsID_upper = -(1 as libc::c_int);
                    } else {
                        found_upper = 1 as libc::c_int;
                        nBits_upper = nBits;
                        gainMult_upper = gainMult_Q8 as opus_int32;
                        gainsID_upper = gainsID;
                    }
                } else {
                    if !(nBits < maxBits - 5 as libc::c_int) {
                        break;
                    }
                    found_lower = 1 as libc::c_int;
                    nBits_lower = nBits;
                    gainMult_lower = gainMult_Q8 as opus_int32;
                    if gainsID != gainsID_lower {
                        gainsID_lower = gainsID;
                        memcpy(
                            &mut sRangeEnc_copy2 as *mut ec_enc as *mut libc::c_void,
                            psRangeEnc as *const libc::c_void,
                            ::core::mem::size_of::<ec_enc>() as libc::c_ulong,
                        );
                        if !((*psRangeEnc).offs <= 1275 as libc::c_int as libc::c_uint) {
                            celt_fatal(
                                b"assertion failed: psRangeEnc->offs <= 1275\0" as *const u8
                                    as *const libc::c_char,
                                b"silk/float/encode_frame_FLP.c\0" as *const u8
                                    as *const libc::c_char,
                                281 as libc::c_int,
                            );
                        }
                        memcpy(
                            ec_buf_copy.as_mut_ptr() as *mut libc::c_void,
                            (*psRangeEnc).buf as *const libc::c_void,
                            (*psRangeEnc).offs as libc::c_ulong,
                        );
                        memcpy(
                            &mut sNSQ_copy2 as *mut silk_nsq_state as *mut libc::c_void,
                            &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state
                                as *const libc::c_void,
                            ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
                        );
                        LastGainIndex_copy2 = (*psEnc).sShape.LastGainIndex;
                    }
                }
                if found_lower == 0 && nBits > maxBits {
                    let mut j: libc::c_int = 0;
                    i = 0 as libc::c_int;
                    while i < (*psEnc).sCmn.nb_subfr {
                        let mut sum: libc::c_int = 0 as libc::c_int;
                        j = i * (*psEnc).sCmn.subfr_length;
                        while j < (i + 1 as libc::c_int) * (*psEnc).sCmn.subfr_length {
                            sum += abs((*psEnc).sCmn.pulses[j as usize] as libc::c_int);
                            j += 1;
                        }
                        if iter == 0 as libc::c_int
                            || sum < best_sum[i as usize] && gain_lock[i as usize] == 0
                        {
                            best_sum[i as usize] = sum;
                            best_gain_mult[i as usize] = gainMult_Q8;
                        } else {
                            gain_lock[i as usize] = 1 as libc::c_int;
                        }
                        i += 1;
                    }
                }
                if found_lower & found_upper == 0 as libc::c_int {
                    if nBits > maxBits {
                        if (gainMult_Q8 as libc::c_int) < 16384 as libc::c_int {
                            gainMult_Q8 = (gainMult_Q8 as libc::c_int * 2 as libc::c_int)
                                as opus_int16;
                        } else {
                            gainMult_Q8 = 32767 as libc::c_int as opus_int16;
                        }
                    } else {
                        let mut gain_factor_Q16: opus_int32 = 0;
                        gain_factor_Q16 = silk_log2lin(
                            (((nBits - maxBits) as opus_uint32) << 7 as libc::c_int)
                                as opus_int32 / (*psEnc).sCmn.frame_length
                                + ((16 as libc::c_int as libc::c_long
                                    * ((1 as libc::c_int as opus_int64) << 7 as libc::c_int))
                                    as libc::c_double + 0.5f64) as opus_int32,
                        );
                        gainMult_Q8 = (gain_factor_Q16 as libc::c_long
                            * gainMult_Q8 as opus_int64 >> 16 as libc::c_int)
                            as opus_int32 as opus_int16;
                    }
                } else {
                    gainMult_Q8 = (gainMult_lower
                        + (gainMult_upper - gainMult_lower) * (maxBits - nBits_lower)
                            / (nBits_upper - nBits_lower)) as opus_int16;
                    if gainMult_Q8 as libc::c_int
                        > gainMult_lower
                            + (gainMult_upper - gainMult_lower >> 2 as libc::c_int)
                    {
                        gainMult_Q8 = (gainMult_lower
                            + (gainMult_upper - gainMult_lower >> 2 as libc::c_int))
                            as opus_int16;
                    } else if (gainMult_Q8 as libc::c_int)
                        < gainMult_upper
                            - (gainMult_upper - gainMult_lower >> 2 as libc::c_int)
                    {
                        gainMult_Q8 = (gainMult_upper
                            - (gainMult_upper - gainMult_lower >> 2 as libc::c_int))
                            as opus_int16;
                    }
                }
                i = 0 as libc::c_int;
                while i < (*psEnc).sCmn.nb_subfr {
                    let mut tmp: opus_int16 = 0;
                    if gain_lock[i as usize] != 0 {
                        tmp = best_gain_mult[i as usize];
                    } else {
                        tmp = gainMult_Q8;
                    }
                    pGains_Q16[i
                        as usize] = (((if 0x80000000 as libc::c_uint as opus_int32
                        >> 8 as libc::c_int
                        > 0x7fffffff as libc::c_int >> 8 as libc::c_int
                    {
                        (if (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                            * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32
                            > 0x80000000 as libc::c_uint as opus_int32
                                >> 8 as libc::c_int
                        {
                            0x80000000 as libc::c_uint as opus_int32 >> 8 as libc::c_int
                        } else {
                            (if ((sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                                * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32)
                                < 0x7fffffff as libc::c_int >> 8 as libc::c_int
                            {
                                0x7fffffff as libc::c_int >> 8 as libc::c_int
                            } else {
                                (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                                    * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32
                            })
                        })
                    } else {
                        (if (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                            * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32
                            > 0x7fffffff as libc::c_int >> 8 as libc::c_int
                        {
                            0x7fffffff as libc::c_int >> 8 as libc::c_int
                        } else {
                            (if ((sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                                * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32)
                                < 0x80000000 as libc::c_uint as opus_int32
                                    >> 8 as libc::c_int
                            {
                                0x80000000 as libc::c_uint as opus_int32 >> 8 as libc::c_int
                            } else {
                                (sEncCtrl.GainsUnq_Q16[i as usize] as libc::c_long
                                    * tmp as opus_int64 >> 16 as libc::c_int) as opus_int32
                            })
                        })
                    }) as opus_uint32) << 8 as libc::c_int) as opus_int32;
                    i += 1;
                }
                (*psEnc).sShape.LastGainIndex = sEncCtrl.lastGainIndexPrev;
                silk_gains_quant(
                    ((*psEnc).sCmn.indices.GainsIndices).as_mut_ptr(),
                    pGains_Q16.as_mut_ptr(),
                    &mut (*psEnc).sShape.LastGainIndex,
                    (condCoding == CODE_CONDITIONALLY) as libc::c_int,
                    (*psEnc).sCmn.nb_subfr,
                );
                gainsID = silk_gains_ID(
                    ((*psEnc).sCmn.indices.GainsIndices).as_mut_ptr()
                        as *const opus_int8,
                    (*psEnc).sCmn.nb_subfr,
                );
                i = 0 as libc::c_int;
                while i < (*psEnc).sCmn.nb_subfr {
                    sEncCtrl
                        .Gains[i
                        as usize] = pGains_Q16[i as usize] as libc::c_float / 65536.0f32;
                    i += 1;
                }
                iter += 1;
            }
        }
    }
    memmove(
        ((*psEnc).x_buf).as_mut_ptr() as *mut libc::c_void,
        &mut *((*psEnc).x_buf).as_mut_ptr().offset((*psEnc).sCmn.frame_length as isize)
            as *mut libc::c_float as *const libc::c_void,
        (((*psEnc).sCmn.ltp_mem_length + 5 as libc::c_int * (*psEnc).sCmn.fs_kHz)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    if (*psEnc).sCmn.prefillFlag != 0 {
        *pnBytesOut = 0 as libc::c_int;
        return ret;
    }
    (*psEnc)
        .sCmn
        .prevLag = sEncCtrl.pitchL[((*psEnc).sCmn.nb_subfr - 1 as libc::c_int) as usize];
    (*psEnc).sCmn.prevSignalType = (*psEnc).sCmn.indices.signalType;
    (*psEnc).sCmn.first_frame_after_reset = 0 as libc::c_int;
    *pnBytesOut = ec_tell(psRangeEnc) + 7 as libc::c_int >> 3 as libc::c_int;
    return ret;
}
#[inline]
#[c2rust::src_loc = "382:1"]
unsafe extern "C" fn silk_LBRR_encode_FLP(
    mut psEnc: *mut silk_encoder_state_FLP,
    mut psEncCtrl: *mut silk_encoder_control_FLP,
    mut xfw: *const libc::c_float,
    mut condCoding: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut Gains_Q16: [opus_int32; 4] = [0; 4];
    let mut TempGains: [libc::c_float; 4] = [0.; 4];
    let mut psIndices_LBRR: *mut SideInfoIndices = &mut *((*psEnc).sCmn.indices_LBRR)
        .as_mut_ptr()
        .offset((*psEnc).sCmn.nFramesEncoded as isize) as *mut SideInfoIndices;
    let mut sNSQ_LBRR: silk_nsq_state = silk_nsq_state {
        xq: [0; 640],
        sLTP_shp_Q14: [0; 640],
        sLPC_Q14: [0; 96],
        sAR2_Q14: [0; 24],
        sLF_AR_shp_Q14: 0,
        sDiff_shp_Q14: 0,
        lagPrev: 0,
        sLTP_buf_idx: 0,
        sLTP_shp_buf_idx: 0,
        rand_seed: 0,
        prev_gain_Q16: 0,
        rewhite_flag: 0,
    };
    if (*psEnc).sCmn.LBRR_enabled != 0
        && (*psEnc).sCmn.speech_activity_Q8
            > ((0.3f32
                * ((1 as libc::c_int as opus_int64) << 8 as libc::c_int)
                    as libc::c_float) as libc::c_double + 0.5f64) as opus_int32
    {
        (*psEnc)
            .sCmn
            .LBRR_flags[(*psEnc).sCmn.nFramesEncoded as usize] = 1 as libc::c_int;
        memcpy(
            &mut sNSQ_LBRR as *mut silk_nsq_state as *mut libc::c_void,
            &mut (*psEnc).sCmn.sNSQ as *mut silk_nsq_state as *const libc::c_void,
            ::core::mem::size_of::<silk_nsq_state>() as libc::c_ulong,
        );
        memcpy(
            psIndices_LBRR as *mut libc::c_void,
            &mut (*psEnc).sCmn.indices as *mut SideInfoIndices as *const libc::c_void,
            ::core::mem::size_of::<SideInfoIndices>() as libc::c_ulong,
        );
        memcpy(
            TempGains.as_mut_ptr() as *mut libc::c_void,
            ((*psEncCtrl).Gains).as_mut_ptr() as *const libc::c_void,
            ((*psEnc).sCmn.nb_subfr as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        if (*psEnc).sCmn.nFramesEncoded == 0 as libc::c_int
            || (*psEnc)
                .sCmn
                .LBRR_flags[((*psEnc).sCmn.nFramesEncoded - 1 as libc::c_int) as usize]
                == 0 as libc::c_int
        {
            (*psEnc).sCmn.LBRRprevLastGainIndex = (*psEnc).sShape.LastGainIndex;
            (*psIndices_LBRR)
                .GainsIndices[0 as libc::c_int
                as usize] = ((*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize]
                as libc::c_int + (*psEnc).sCmn.LBRR_GainIncreases) as opus_int8;
            (*psIndices_LBRR)
                .GainsIndices[0 as libc::c_int
                as usize] = silk_min_int(
                (*psIndices_LBRR).GainsIndices[0 as libc::c_int as usize] as libc::c_int,
                N_LEVELS_QGAIN - 1 as libc::c_int,
            ) as opus_int8;
        }
        silk_gains_dequant(
            Gains_Q16.as_mut_ptr(),
            ((*psIndices_LBRR).GainsIndices).as_mut_ptr() as *const opus_int8,
            &mut (*psEnc).sCmn.LBRRprevLastGainIndex,
            (condCoding == CODE_CONDITIONALLY) as libc::c_int,
            (*psEnc).sCmn.nb_subfr,
        );
        k = 0 as libc::c_int;
        while k < (*psEnc).sCmn.nb_subfr {
            (*psEncCtrl)
                .Gains[k
                as usize] = Gains_Q16[k as usize] as libc::c_float
                * (1.0f32 / 65536.0f32);
            k += 1;
        }
        silk_NSQ_wrapper_FLP(
            psEnc,
            psEncCtrl,
            psIndices_LBRR,
            &mut sNSQ_LBRR,
            ((*psEnc).sCmn.pulses_LBRR[(*psEnc).sCmn.nFramesEncoded as usize])
                .as_mut_ptr(),
            xfw,
        );
        memcpy(
            ((*psEncCtrl).Gains).as_mut_ptr() as *mut libc::c_void,
            TempGains.as_mut_ptr() as *const libc::c_void,
            ((*psEnc).sCmn.nb_subfr as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
    }
}
