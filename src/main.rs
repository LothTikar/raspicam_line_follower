extern crate rscam;

fn main() {
    let width: usize = 64;
    let height: usize = 48;

    let mut camera = rscam::new("/dev/video0").unwrap();

    camera
        .start(&rscam::Config {
            interval: (1, 3), // 30 fps.
            resolution: (width as u32, height as u32),
            format: b"RGB3",
            ..Default::default()
        })
        .unwrap();
    loop {
        let frame = camera.capture().unwrap();

        for row in frame.chunks(width*3) {
            let mut vec = String::new();
            for pixel in row.chunks(3) {
                let mut is_black = true;
                for color in pixel {
                    if *color > 60 {
                        is_black = false;
                    }
                }
                if is_black {
                    vec.push('0');
                } else {
                    vec.push(' ');
                }
            }
            println!("{}", vec);
        }
        println!("");
    }
}
