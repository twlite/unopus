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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "51:4"]
    pub type opus_int8 = int8_t;
    #[c2rust::src_loc = "52:4"]
    pub type opus_uint8 = uint8_t;
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    use super::stdint_intn_h::{int16_t, int32_t, int8_t};
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
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
    use super::opus_types_h::{opus_int16, opus_int32};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
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
    use super::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
    use super::resampler_structs_h::silk_resampler_state_struct;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/errors.h:32"]
pub mod errors_h {
    #[c2rust::src_loc = "39:9"]
    pub const SILK_NO_ERROR: libc::c_int = 0 as libc::c_int;
}
pub use self::errors_h::SILK_NO_ERROR;
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int8, opus_uint8};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int8_t};
pub use self::stdint_uintn_h::uint8_t;
pub use self::structs_h::{
    silk_LP_state, silk_NLSF_CB_struct, silk_VAD_state, silk_encoder_state, silk_nsq_state,
    SideInfoIndices,
};
pub use self::types_h::{__int16_t, __int32_t, __int8_t, __uint8_t};
#[c2rust::src_loc = "40:28"]
static mut silk_TargetRate_NB_21: [libc::c_uchar; 107] = [
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    68 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    79 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "51:28"]
static mut silk_TargetRate_MB_21: [libc::c_uchar; 155] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    109 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "65:28"]
static mut silk_TargetRate_WB_21: [libc::c_uchar; 191] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    77 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    83 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    116 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn silk_control_SNR(
    mut psEncC: *mut silk_encoder_state,
    mut TargetRate_bps: opus_int32,
) -> libc::c_int {
    let mut id: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut snr_table: *const libc::c_uchar = 0 as *const libc::c_uchar;
    (*psEncC).TargetRate_bps = TargetRate_bps;
    if (*psEncC).nb_subfr == 2 as libc::c_int {
        TargetRate_bps -= 2000 as libc::c_int + (*psEncC).fs_kHz / 16 as libc::c_int;
    }
    if (*psEncC).fs_kHz == 8 as libc::c_int {
        bound = ::core::mem::size_of::<[libc::c_uchar; 107]>() as libc::c_ulong as libc::c_int;
        snr_table = silk_TargetRate_NB_21.as_ptr();
    } else if (*psEncC).fs_kHz == 12 as libc::c_int {
        bound = ::core::mem::size_of::<[libc::c_uchar; 155]>() as libc::c_ulong as libc::c_int;
        snr_table = silk_TargetRate_MB_21.as_ptr();
    } else {
        bound = ::core::mem::size_of::<[libc::c_uchar; 191]>() as libc::c_ulong as libc::c_int;
        snr_table = silk_TargetRate_WB_21.as_ptr();
    }
    id = (TargetRate_bps + 200 as libc::c_int) / 400 as libc::c_int;
    id = if (id - 10 as libc::c_int) < bound - 1 as libc::c_int {
        id - 10 as libc::c_int
    } else {
        bound - 1 as libc::c_int
    };
    if id <= 0 as libc::c_int {
        (*psEncC).SNR_dB_Q7 = 0 as libc::c_int;
    } else {
        (*psEncC).SNR_dB_Q7 = *snr_table.offset(id as isize) as libc::c_int * 21 as libc::c_int;
    }
    return SILK_NO_ERROR;
}
