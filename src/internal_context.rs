use crate::{Config, Context};
use ffmpeg_next::{
    ffi::*,
    format::{sample::Type as SampleType, Pixel, Sample},
    frame::{Audio, Video},
    picture::Type as PictureType,
    software::scaling::Flags,
    ChannelLayout,
};
use std::{ffi::c_void, ptr::null_mut};

pub(crate) struct InternalContext<Ctx: Context> {
    c: Config,
    ctx: Ctx,
    h264sws: Option<*mut SwsContext>,
}

impl<Ctx: Context> InternalContext<Ctx> {
    #[inline(always)]
    pub(crate) fn new(c: Config, ctx: Ctx) -> Self {
        Self {
            c,
            ctx,
            h264sws: None,
        }
    }

    #[inline(always)]
    pub(crate) fn width(&self) -> i32 {
        self.c.width
    }
    #[inline(always)]
    pub(crate) fn height(&self) -> i32 {
        self.c.height
    }

    #[inline(always)]
    pub(crate) fn on_paint(&self, data: *const c_void) {
        if self.c.raw_handler {
            self.ctx.on_video_raw(data);
        }
    }
    #[inline(always)]
    pub(crate) fn on_audio_stream_packet(&self, data: *mut *const f32, samples: i32, pts: i64) {
        if self.c.raw_handler {
            self.ctx.on_audio_raw(data, samples, pts);
        }
    }

    #[inline(always)]
    unsafe fn get_h264sws(&mut self) -> *mut SwsContext {
        if self.h264sws.is_none() {
            self.h264sws = Some(sws_getContext(
                self.c.width,
                self.c.height,
                Pixel::BGRA.into(),
                self.c.width,
                self.c.height,
                Pixel::YUVA420P.into(),
                Flags::BICUBIC.bits(),
                null_mut(),
                null_mut(),
                null_mut(),
            ));
        }
        self.h264sws.unwrap()
    }
    #[inline(always)]
    unsafe fn h264vframe(&mut self, buffer: *const c_void) -> Video {
        let width: u32 = 1920;
        let height: u32 = 1080;
        let mut frame = Video::new(Pixel::YUVA420P, width, height);
        frame.set_kind(PictureType::None);
        frame.set_pts(Some(av_gettime()));
        let ptr = frame.as_mut_ptr();
        sws_scale(
            self.get_h264sws(),
            &[buffer as *const u8] as _,
            &[4 * width as i32] as _,
            0,
            height as i32,
            (*ptr).data.as_ptr(),
            (*ptr).linesize.as_ptr(),
        );
        frame
    }

    #[inline(always)]
    unsafe fn aframe(&mut self, data: *mut *const f32, samples: i32, pts: i64) -> Audio {
        let mut frame = Audio::new(
            Sample::F32(SampleType::Packed),
            samples as usize,
            ChannelLayout::STEREO,
        );
        frame.set_pts(Some(pts * 1000));
        let ptr = frame.as_mut_ptr();
        for i in 0..=(AV_NUM_DATA_POINTERS as usize) {
            (*ptr).data[i] = data.offset(i as _) as _;
        }
        frame
    }
}

impl<Ctx: Context> Drop for InternalContext<Ctx> {
    fn drop(&mut self) {
        unsafe {
            if let Some(h264sws) = self.h264sws {
                sws_freeContext(h264sws);
            }
        }
    }
}
