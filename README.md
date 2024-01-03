# cellulose_fiber

Chromium Embedded Framework in Rust, focusing on windowless stream

This project does not aim to create a cross platform safe Rust bindings of CEF api. Instead it will focus on:

- [x] Linux only
- [x] Automated script to download CEF binary from cdn such as Spotify
- [ ] Automated script to build CEF with proprietary codecs or custom features
- [x] Automated build process to generate rust bindings using bindgen with CEF capi
- [x] Automated script to install dynamic linking libraries and objects
- [x] Any generatable resource such as bindings or CEF build should be ignored from git history since they can be generated locally
- [x] Rust api to receive render stream with audio using windowless (offscreen) mode
- [ ] Rust api to save the stream to a mkv file with raw encoding
- [ ] Rust api to save the stream to a mp4 file with h264 encoding
- [ ] Rust api to save the stream to a file with any encoding
- [ ] Rust api to push the stream to a rtmp server
- [ ] Rust api to push the stream to a srt server (require custom ffmpeg build)
- [ ] Automated script to create release distribution binary with dynamic linking
- [ ] Documentation on how to use this as a crate

The stream will be processed in rust using [ffmpeg-next](https://crates.io/crates/ffmpeg-next). Since the cef bindings and also the ffmpeg bindings are `unsafe`, thus we will mark all api exposed by this crate as `unsafe`

### Examples

```rs
use cellulose_fiber::{start, Config, Context};

struct MyContext;
impl Context for MyContext {}

fn main() {
    unsafe {
        start(
            Config {
                url: "https://nongdan.dev/bunny.mp4?autoplay=1".to_string(),
                mp4_output: "output.mp4".into(),
                ..Default::default()
            },
            MyContext {},
        );
    }
}
```

- [All examples](examples)

### Contribution

```sh
make download

# rustc dynamic link dependencies
sudo apt-get install -y libclang-dev
make rebuild

# cef dynamic link dependencies
sudo apt-get install -y libnss3-tools libatk1.0-0 libatk-bridge2.0-0 libcups2 libxcomposite-dev

# ffpmeg dependencies
sudo apt-get install -y clang libavcodec-dev libavformat-dev libavutil-dev libavfilter-dev libavdevice-dev pkg-config

sudo apt-get install -y xvfb
Xvfb :0 -ac -screen 0 1920x1080x24+32 &

export DISPLAY=:0
cargo run --example raw_handler --features debug
```
