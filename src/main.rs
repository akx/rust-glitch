extern crate image;
use std::os;
use std::io::File;
use std::rand;
use std::rand::Rng;
use image::GenericImage;

fn clamp(val: int, min: int, max: int) -> int {
    if val <= min { return min; }
    if val >= max { return max; }
    return val;
}

fn main() {
    let filename = if os::args().len() == 2 {
        os::args().as_slice()[1].clone()
    } else {
        panic!("Please pass a filename.")
    };
    
    let mut buf = image::open(&Path::new(&filename)).unwrap().to_rgb();
    println!("dimensions {}", buf.dimensions());
    
    let (w, h) = buf.dimensions();
    let mut xoff = 0i;
    let mut yoff = 0i;
    let mut rng = rand::task_rng();
    
    for y in range(0, h) {
        for x in range(0, w) {
            if rand::random::<u16>() < 100 {
                xoff += rng.gen_range(-1i, 2);
            }
            if rand::random::<u16>() < 500 {
                yoff += rng.gen_range(-1i, 2);
            }
            if rand::random::<u16>() < 10 {
                xoff /= 2;
                yoff /= 2;
            }
            
            let srcx = clamp(((x as int) + xoff), 0, (w - 1) as int);
            let srcy = clamp(((y as int) + yoff), 0, (h - 1) as int);
            let srcpx = buf[(srcx as u32, srcy as u32)];
            buf.put_pixel(x, y, srcpx);
        }
    }
    
    let out_filename = format!("{}.rg.png", filename);
    let fout = File::create(&Path::new(&out_filename)).unwrap();
    let _ = image::ImageRgb8(buf).save(fout, image::PNG);
    println!("Saved => {0}", out_filename);
}
