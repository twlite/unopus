#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::varargs::VarArgs;

pub struct Encoder([u8; 1]);
pub struct Decoder([u8; 1]);

pub trait OpusBackend {
    unsafe fn opus_encoder_create(
        &self,
        Fs: i32,
        channels: i32,
        application: i32,
        error: *mut i32,
    ) -> *mut Encoder;
    unsafe fn opus_encoder_ctl_impl(&self, st: *mut Encoder, request: i32, args: VarArgs) -> i32;
    unsafe fn opus_encode(
        &self,
        st: *mut Encoder,
        pcm: *const i16,
        analysis_frame_size: i32,
        data: *mut u8,
        max_data_bytes: i32,
    ) -> i32;
    unsafe fn opus_encoder_destroy(&self, st: *mut Encoder);

    unsafe fn opus_decoder_create(&self, Fs: i32, channels: i32, error: *mut i32) -> *mut Decoder;
    unsafe fn opus_decode(
        &self,
        st: *mut Decoder,
        data: *const u8,
        len: i32,
        pcm: *mut i16,
        frame_size: i32,
        decode_fec: i32,
    ) -> i32;
    unsafe fn opus_decoder_ctl_impl(&self, st: *mut Decoder, request: i32, args: VarArgs) -> i32;
    unsafe fn opus_decoder_destroy(&self, st: *mut Decoder);
}

mod unopus {
    use crate::test::demo::backend::{Decoder, Encoder};
    use crate::varargs::VarArgs;
    use crate::{
        opus_decode, opus_decoder_create, opus_decoder_ctl_impl, opus_decoder_destroy, opus_encode,
        opus_encoder_create, opus_encoder_ctl_impl, opus_encoder_destroy,
    };

    pub struct UnsafeLibopusBackend;

    impl super::OpusBackend for UnsafeLibopusBackend {
        unsafe fn opus_encoder_create(
            &self,
            Fs: i32,
            channels: i32,
            application: i32,
            error: *mut i32,
        ) -> *mut Encoder {
            opus_encoder_create(Fs, channels, application, error) as *mut _
        }

        unsafe fn opus_encoder_ctl_impl(
            &self,
            st: *mut Encoder,
            request: i32,
            args: VarArgs,
        ) -> i32 {
            opus_encoder_ctl_impl(st as *mut _, request, args)
        }

        unsafe fn opus_encode(
            &self,
            st: *mut Encoder,
            pcm: *const i16,
            analysis_frame_size: i32,
            data: *mut u8,
            max_data_bytes: i32,
        ) -> i32 {
            opus_encode(st as *mut _, pcm, analysis_frame_size, data, max_data_bytes)
        }

        unsafe fn opus_encoder_destroy(&self, st: *mut Encoder) {
            opus_encoder_destroy(st as *mut _)
        }

        unsafe fn opus_decoder_create(
            &self,
            Fs: i32,
            channels: i32,
            error: *mut i32,
        ) -> *mut Decoder {
            opus_decoder_create(Fs, channels, error) as *mut _
        }

        unsafe fn opus_decode(
            &self,
            st: *mut Decoder,
            data: *const u8,
            len: i32,
            pcm: *mut i16,
            frame_size: i32,
            decode_fec: i32,
        ) -> i32 {
            opus_decode(st as *mut _, data, len, pcm, frame_size, decode_fec)
        }

        unsafe fn opus_decoder_ctl_impl(
            &self,
            st: *mut Decoder,
            request: i32,
            args: VarArgs,
        ) -> i32 {
            opus_decoder_ctl_impl(st as *mut _, request, args)
        }

        unsafe fn opus_decoder_destroy(&self, st: *mut Decoder) {
            opus_decoder_destroy(st as *mut _)
        }
    }
}
pub use unopus::UnsafeLibopusBackend;

#[cfg(feature = "test-upstream-libopus")]
mod libopus {
    use crate::test::demo::backend::{Decoder, Encoder};
    use crate::varargs::{VarArg, VarArgs};
    use audiopus_sys::{
        opus_decode, opus_decoder_create, opus_decoder_ctl, opus_decoder_destroy, opus_encode,
        opus_encoder_create, opus_encoder_ctl, opus_encoder_destroy,
    };

    pub struct UpstreamLibopusBackend;

    impl super::OpusBackend for UpstreamLibopusBackend {
        unsafe fn opus_encoder_create(
            &self,
            Fs: i32,
            channels: i32,
            application: i32,
            error: *mut i32,
        ) -> *mut Encoder {
            opus_encoder_create(Fs, channels, application, error) as *mut _
        }

        unsafe fn opus_encoder_ctl_impl(
            &self,
            st: *mut Encoder,
            request: i32,
            mut args: VarArgs,
        ) -> i32 {
            match &mut args.0[..] {
                [VarArg::I32(arg)] => opus_encoder_ctl(st as *mut _, request, *arg),
                [VarArg::I32Out(arg)] => opus_encoder_ctl(st as *mut _, request, *arg as *mut _),
                [VarArg::U32Out(arg)] => opus_encoder_ctl(st as *mut _, request, *arg as *mut _),
                // manually match over all required signatures
                _ => todo!("opus_decoder_ctl signature not implemented"),
            }
        }

        unsafe fn opus_encode(
            &self,
            st: *mut Encoder,
            pcm: *const i16,
            analysis_frame_size: i32,
            data: *mut u8,
            max_data_bytes: i32,
        ) -> i32 {
            opus_encode(st as *mut _, pcm, analysis_frame_size, data, max_data_bytes)
        }

        unsafe fn opus_encoder_destroy(&self, st: *mut Encoder) {
            opus_encoder_destroy(st as *mut _)
        }

        unsafe fn opus_decoder_create(
            &self,
            Fs: i32,
            channels: i32,
            error: *mut i32,
        ) -> *mut Decoder {
            opus_decoder_create(Fs, channels, error) as *mut _
        }

        unsafe fn opus_decode(
            &self,
            st: *mut Decoder,
            data: *const u8,
            len: i32,
            pcm: *mut i16,
            frame_size: i32,
            decode_fec: i32,
        ) -> i32 {
            opus_decode(st as *mut _, data, len, pcm, frame_size, decode_fec)
        }

        unsafe fn opus_decoder_ctl_impl(
            &self,
            st: *mut Decoder,
            request: i32,
            mut args: VarArgs,
        ) -> i32 {
            match &mut args.0[..] {
                // manually match over all required signatures
                [VarArg::U32Out(ptr)] => opus_decoder_ctl(st as *mut _, request, *ptr as *mut _),
                _ => todo!("opus_decoder_ctl signature not implemented"),
            }
        }

        unsafe fn opus_decoder_destroy(&self, st: *mut Decoder) {
            opus_decoder_destroy(st as *mut _)
        }
    }
}
#[cfg(feature = "test-upstream-libopus")]
pub use libopus::UpstreamLibopusBackend;
