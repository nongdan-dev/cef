use crate::{
    audio_handler::AudioHandler,
    ffi::*,
    internal_context::InternalContext,
    println_debug,
    ptr::{from_ptr, Brc, PhantomWrapper},
    render_handler::RenderHandler,
    Context,
};
use std::sync::Arc;

#[repr(C)]
pub(crate) struct Client<Ctx: Context> {
    // first member has the same C pointer with this wrapper struct
    v: cef_client_t,
    ctx: Arc<InternalContext<Ctx>>,
}
impl<Ctx: Context> PhantomWrapper<cef_client_t> for Client<Ctx> {
    // phantom trait to store cef typing
}
impl<Ctx: Context> Client<Ctx> {
    unsafe extern "C" fn get_render_handler(_ptr: *mut cef_client_t) -> *mut cef_render_handler_t {
        println_debug!("get_render_handler");
        let this: &mut Self = from_ptr(_ptr);
        RenderHandler::new(this.ctx.clone())
    }
    unsafe extern "C" fn get_audio_handler(_ptr: *mut cef_client_t) -> *mut cef_audio_handler_t {
        println_debug!("get_audio_handler");
        let this: &mut Self = from_ptr(_ptr);
        AudioHandler::new(this.ctx.clone())
    }
    pub(crate) fn new(ctx: Arc<InternalContext<Ctx>>) -> *mut cef_client_t {
        Brc::new(
            |base| Self {
                v: cef_client_t {
                    base,
                    get_render_handler: Some(Self::get_render_handler),
                    get_audio_handler: Some(Self::get_audio_handler),
                    ..Default::default()
                },
                ctx,
            },
            #[cfg(feature = "debug")]
            "client".to_string(),
        )
    }
}
