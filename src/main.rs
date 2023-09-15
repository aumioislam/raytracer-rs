use std::io::{self, Write};

pub mod array3;
pub mod util;

use crate::util::*;

fn main() -> io::Result<()> {
    let mut cout = io::stdout().lock();
    let mut cerr = io::stderr().lock();

    // image
    let width = 256;
    let height = 256;

    // render
    write!(&mut cout, "P3\n{} {}\n255\n", width, height)?;
    for i in 0..height {
        write!(&mut cerr, "Scanlines remaining: {}\n", height-i)?;
        cerr.flush()?;
        for j in 0..width {
            let r = j as f64 / (width as f64 - 1.0);
            let g = i as f64 / (height as f64 - 1.0);
            let b = 0.0;

            let pix = Pixel::new([r,g,b,]);

            write_pixel(&mut cout, pix)?;
        }
    }

    write!(&mut cerr, "Done\n")?;
    cerr.flush()?;
    Ok(())
}
