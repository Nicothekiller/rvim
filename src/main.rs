use rvim::{Buffer, TuiRenderer};

fn main() {
    let buffer = Buffer::new(String::from("src/buffer.rs"));
    let mut renderer = TuiRenderer::new(buffer);
    renderer.run();
}
