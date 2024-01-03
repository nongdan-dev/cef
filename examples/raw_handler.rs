use cellulose_fiber::{start, Config, Context};
use std::ffi::c_void;

struct MyContext;
impl Context for MyContext {
    #[allow(unused_variables)]
    fn on_video_raw(&self, data: *const c_void) {
        println!("got raw video frame");
    }
    #[allow(unused_variables)]
    fn on_audio_raw(&self, data: *mut *const f32, samples: i32, pts: i64) {
        println!("got raw audio frame");
    }
}

fn main() {
    unsafe {
        start(
            Config {
                url: "https://www.youtube.com/embed/hKRUPYrAQoE?autoplay=1".to_string(),
                raw_handler: true,
                ..Default::default()
            },
            MyContext {},
        );
    }
}
