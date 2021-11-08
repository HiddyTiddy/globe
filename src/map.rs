use ang::atan2;

use crate::Point3;
use std::{f32::consts::PI, fs::File};

pub struct Map {
    buffer: Vec<u8>,
    height: u32,
    width: u32,
}

const FACTOR: f32 = 1.0 / 255.0;

impl Map {
    /*
     * creates new Map struct from grayscale image
     * i think this works fine
     */
    pub fn new(filename: &str) -> Map {
        let decoder = png::Decoder::new(File::open(filename).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        Map {
            buffer: Vec::from(bytes),
            height: info.height,
            width: info.width,
        }
    }

    /*
     * helper function to index image
     */
    fn at(&self, x: u32, y: u32) -> u8 {
        let idx = (x + self.width * y) as usize;
        assert!(x < self.width && y < self.height);
        if idx as usize >= self.buffer.len() {
            panic!("out of bounds with x={} and y={}", x, y);
        } else {
            self.buffer[idx]
        }
    }

    /*
     * returns height as given by color value of the image at the specified point.
     * this does not yet work properly
     */
    pub fn height_at_point(&self, point: Point3<f32>) -> f32 {
        let (x, y) = self.point_to_coordinate(point);
        // println!("{} {}", lat, lon);

        let val = self.at(x, y);

        // println!(
        //     "{} {} {} -> {} {} -> {}",
        //     point.x,
        //     point.y,
        //     point.z,
        //     x,
        //     y,
        //     if val == 0xff {
        //         1.0
        //     } else {
        //         1.0 + (val as f32) * FACTOR
        //     }
        // ); // dump

        // println!("{} {} {:?}", x,y, rgb);

        if val == 0xff {
            1.0
        } else {
            //1.0 + (val as f32) * FACTOR
            1.1
        }
    }

    // fn dumb_point_to_coordinate(&self, point: Point3<f32>) -> (u32, u32) {
    //     let lat = atan2(point.y, point.x).in_radians(); // -pi/2 to pi/2
    //     let lon: f32 = if point.x != 0.0 {
    //         (point.z / point.x).atan()
    //     } else if point.z > 0. {
    //             PI / 2.
    //         } else {
    //             -PI / 2.
    //     }; // -pi to pi
    //        // let y = lat.tan().

    //     let x:f32 = lat / PI +0.5;
    //     let y:f32 = lon / PI / 2.0 + 0.5;

    //     ((x * self.width as f32) as u32, (y * self.height as f32) as u32)
    // }

    fn point_to_coordinate(&self, point: Point3<f32>) -> (u32, u32) {
        // let lat = point.z.acos(); // -pi/2 to pi/2
        // let lon: f32 = ang::atan2(point.x, point.y).in_radians();
        // let y = lat.tan().

        let (lat, lon) = point_to_coord(point);
        // println!("{} {}", lat, lon);

        let y: f32 = (lon + PI) / 2.0 / PI; //((lat + PI) % (2.*PI)) / PI * 0.5;
        let x: f32 = dumb_y(lat);
        // println!("{} <> {}", lat, y);
        // let y = y / 3.800_201_2 / 2.0 + 0.5;

        (
            (x * self.width as f32) as u32,
            (y * self.height as f32) as u32,
        )
    }
}

const LIMIT: f32 = 1.526_071_1;

fn mercator_y(latitude: f32) -> f32 {
    // if latitude > LIMIT {
    //     latitude = LIMIT - 0.0001;
    // } else if latitude < -LIMIT {
    //     latitude = -LIMIT + 0.0001;
    // }
    // let tan = (latitude / 2.0 + PI / 4.).tan();
    // tan.ln() /  / 2.0 + 0.5
    if !(-LIMIT..=LIMIT).contains(&latitude) {
        latitude / PI
    } else {
        let val = (latitude / 2.0 + PI / 4.).tan().ln();
        val / (LIMIT / 2.0 + PI / 4.0).tan().ln() * 0.4 + 0.5
    }
}

fn dumb_y(latitude: f32) -> f32 {
    latitude / PI
}

// yanked from https://www.youtube.com/watch?v=sLqXFF8mlEU
fn point_to_coord(point: Point3<f32>) -> (f32, f32) {
    let longitude = point.y.asin();
    let latitude = atan2(point.x, point.z).in_radians();
    (latitude, longitude)
}

// fn coordinate_to_point(lat: f32, lon: f32) -> Point3<f32>{
//     let y = lat.sin();
//     let r = lat.cos();
//     let x = lon.sin() * r;
//     let z = -lon.cos() * r;
//     Point3::new(x,y,z)
// }
