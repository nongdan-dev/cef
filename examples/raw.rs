use cellulose_fiber::{start, Config, Context};

struct MyContext;
impl Context for MyContext {}

fn main() {
    unsafe {
        start(
            Config {
                url: "https://www.youtube.com/embed/hKRUPYrAQoE?autoplay=1".to_string(),
                raw_output: "output.mkv".into(),
                ..Default::default()
            },
            MyContext {},
        );
    }
}
