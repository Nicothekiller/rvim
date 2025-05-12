use rvim::{Buffer, TuiRenderer};

fn main() {
    let buffer = Buffer::new(String::from("testFile.txt"));
    let renderer = TuiRenderer::new(buffer);
    renderer.run();
}
