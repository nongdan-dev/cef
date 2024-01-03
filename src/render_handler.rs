use crate::{
    ffi::*,
    internal_context::InternalContext,
    println_debug,
    ptr::{from_ptr, Brc, PhantomWrapper},
    Context,
};
use std::{
    ffi::{c_int, c_void},
    sync::Arc,
};

#[repr(C)]
pub(crate) struct RenderHandler<Ctx: Context> {
    // first member has the same C pointer with this wrapper struct
    v: cef_render_handler_t,
    ctx: Arc<InternalContext<Ctx>>,
}
impl<Ctx: Context> PhantomWrapper<cef_render_handler_t> for RenderHandler<Ctx> {
    // phantom trait to store cef typing
}
impl<Ctx: Context> RenderHandler<Ctx> {
    #[allow(unused_variables)]
    unsafe extern "C" fn get_view_rect(
        ptr: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        rect: *mut cef_rect_t,
    ) {
        println_debug!("get_view_rect");
        (*rect).x = 0;
        (*rect).y = 0;
        let this: &mut Self = from_ptr(ptr);
        (*rect).width = this.ctx.width();
        (*rect).height = this.ctx.height();
    }
    #[allow(unused_variables)]
    unsafe extern "C" fn on_paint(
        ptr: *mut cef_render_handler_t,
        browser: *mut cef_browser_t,
        ty: cef_paint_element_type_t,
        dirty_rects_count: usize,
        dirty_rects: *const cef_rect_t,
        buffer: *const c_void,
        width: c_int,
        height: c_int,
    ) {
        println_debug!("on_paint");
        if ty > 0 {
            return;
        }
        let this: &mut Self = from_ptr(ptr);
        this.ctx.on_paint(buffer);
    }
    pub(crate) fn new(ctx: Arc<InternalContext<Ctx>>) -> *mut cef_render_handler_t {
        Brc::new(
            |base| Self {
                v: cef_render_handler_t {
                    base,
                    get_view_rect: Some(Self::get_view_rect),
                    on_paint: Some(Self::on_paint),
                    ..Default::default()
                },
                ctx,
            },
            #[cfg(feature = "debug")]
            "render_handler".to_string(),
        )
    }
}
