extern crate rscam;

fn main() {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 48;

    let frame_top: String = ['-'; WIDTH + 2].iter().collect();

    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 3), // 30 fps.
            resolution: (WIDTH as u32, HEIGHT as u32),
            format: b"RGB3",
            ..Default::default()
        })
        .unwrap();
    loop {
        let frame = camera.capture().unwrap();

        println!("{}", frame_top);

        for row in frame.chunks(WIDTH * 3) {
            let mut vec = String::new();
            for pixel in row.chunks(3) {
                let mut is_black = true;
                for color in pixel {
                    if *color > 90 {
                        is_black = false;
                    }
                }
                if is_black {
                    vec.push('0');
                } else {
                    vec.push(' ');
                }
            }
            println!("|{}|", vec);
        }

        println!("{}", frame_top);

        println!("");
    }
}
