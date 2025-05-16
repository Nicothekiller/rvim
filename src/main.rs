use rvim::{FileBuffer, TuiRenderer};

fn main() {
    let buffer = FileBuffer::new(String::from("src/buffer.rs"));
    let mut renderer = TuiRenderer::new(buffer);
    renderer.run();
}
