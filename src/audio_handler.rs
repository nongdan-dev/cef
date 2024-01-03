use crate::{
    ffi::*,
    internal_context::InternalContext,
    println_debug,
    ptr::{from_ptr, Brc, PhantomWrapper},
    Context,
};
use std::{ffi::c_int, sync::Arc};

#[repr(C)]
pub(crate) struct AudioHandler<Ctx: Context> {
    // first member has the same C pointer with this wrapper struct
    v: cef_audio_handler_t,
    ctx: Arc<InternalContext<Ctx>>,
}
impl<Ctx: Context> PhantomWrapper<cef_audio_handler_t> for AudioHandler<Ctx> {
    // phantom trait to store cef typing
}
impl<Ctx: Context> AudioHandler<Ctx> {
    #[allow(unused_variables)]
    unsafe extern "C" fn get_audio_parameters(
        ptr: *mut cef_audio_handler_t,
        browser: *mut cef_browser_t,
        params: *mut cef_audio_parameters_t,
    ) -> c_int {
        println_debug!("get_audio_parameters");
        (*params).channel_layout = cef_channel_layout_t_CEF_CHANNEL_LAYOUT_STEREO;
        (*params).sample_rate = 48000;
        1
    }
    #[allow(unused_variables)]
    unsafe extern "C" fn on_audio_stream_packet(
        ptr: *mut cef_audio_handler_t,
        browser: *mut cef_browser_t,
        data: *mut *const f32,
        frames: c_int,
        pts: i64,
    ) {
        println_debug!("on_audio_stream_packet");
        let this: &mut Self = from_ptr(ptr);
        this.ctx.on_audio_stream_packet(data, frames, pts);
    }
    pub(crate) fn new(ctx: Arc<InternalContext<Ctx>>) -> *mut cef_audio_handler_t {
        Brc::new(
            |base| Self {
                v: cef_audio_handler_t {
                    base,
                    get_audio_parameters: Some(Self::get_audio_parameters),
                    on_audio_stream_packet: Some(Self::on_audio_stream_packet),
                    ..Default::default()
                },
                ctx,
            },
            #[cfg(feature = "debug")]
            "audio_handler".to_string(),
        )
    }
}
