use cellulose_fiber::{start, Config, Context};

struct MyContext;
impl Context for MyContext {}

fn main() {
    unsafe {
        start(
            Config {
                url: "https://www.youtube.com/embed/hKRUPYrAQoE?autoplay=1".to_string(),
                mp4_output: "output.mp4".into(),
                ..Default::default()
            },
            MyContext {},
        );
    }
}
