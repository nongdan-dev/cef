use std::ffi::c_void;

pub trait Context {
    #[allow(unused_variables)]
    fn on_video_raw(&self, data: *const c_void) {
        println!("on_video_raw");
    }
    #[allow(unused_variables)]
    fn on_audio_raw(&self, data: *mut *const f32, samples: i32, pts: i64) {
        println!("on_audio_raw");
    }
}
