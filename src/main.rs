use std::env;

use rvim::{FileBuffer, TuiRenderer};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong amount of arguments used. Please only put one file name at a time.");
        return;
    }

    let buffer = FileBuffer::new(args.pop().unwrap());
    let mut renderer = TuiRenderer::new(buffer);
    renderer.run();
}
