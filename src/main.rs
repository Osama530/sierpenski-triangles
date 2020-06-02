extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 600;

//defining struct for points
struct Points {
    x: u32,
    y: u32,
}

fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut cnt = 1_000_000;

    let pts: [Points; 3] = [
        Points { x: WIDTH / 2, y: 0 },
        Points { x: 0, y: HEIGHT },
        Points {
            x: WIDTH,
            y: HEIGHT,
        },
    ];

    //creating instanse for struct Points
    let mut p = Points {
        x: rand::thread_rng().gen_range(0, WIDTH),
        y: rand::thread_rng().gen_range(0, HEIGHT),
    };

    let pixel = img[(0, 0)];
    while cnt > 0 {
        cnt = cnt - 1;
        let num = rand::thread_rng().gen_range(0, 3);

        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.x + pts[num].y) / 2;

        img.put_pixel(p.x, p.y, pixel);
    }

    //let ref mut fout =File::create("tri.png").unwrap();
    let _ = img.save("tri.png");
}
