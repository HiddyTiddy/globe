use crate::Point3;
use std::{f32::consts::PI, fs::File};

// #[cfg(test)]
// mod tests {
//     use nalgebra::Point3;
//
//     use crate::map;
//
//
//     #[test]
//     fn test_map() {
//         let karte = map::Map::new("data/earth-heightmap.png");
//         let point = Point3::new(0.0, 1.0, 0.0);
//         let (x,y) = map::Map::point_to_coordinate(&karte, point);
//     }
// }
//

pub struct Map {
    buffer: Vec<u8>,
    height: u32,
    width: u32,
}

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
        assert!(
            x < self.width && y < self.height,
            "({} {}) / ({} {}) out of bounds",
            x,
            y,
            self.width,
            self.height
        );
        if idx as usize >= self.buffer.len() {
            panic!("out of bounds with x={} and y={}", x, y);
        } else {
            self.buffer[idx]
        }
    }

    /**
     * input: Point3<f32>, point on the sphere represented in cartesian coordinates
     * output: f32, height at the given point
     */
    pub fn height_at(&self, point: Point3<f32>) -> f32 {
        let point = to_spherical(point);
        let (x, y) = projection(point); // ?
        let x = (x * self.width as f32) as u32;
        let y = (y * self.height as f32) as u32;

        let h = self.at(x, y);
        if h == 0xff {
            1.0
        } else {
            h as f32 / 255.0 / 5.0 + 1.0
            // 1.1
        }
    }
}

/**
 * input: Point3<f32>, point in 3d space, cartesian coordinates, (x,y,z)
 * output: Point3<f32> point in 3d space, spherical coordinates, (r,\theta, \varphi)
 *                      r \in [0, \infty), \theta \in [0, \pi], \varphi \in [0, 2\pi)
 */
fn to_spherical(point: Point3<f32>) -> Point3<f32> {
    let r: f32 = point.x * point.x + point.y * point.y + point.z * point.z;
    let r = r.sqrt();

    let theta: f32 = point.y / r;
    let theta = theta.acos();

    let phi = if point.z == 0.0 {
        PI / 2.0
    } else {
        ang::atan2(point.x, point.z).in_radians()
    };

    Point3::new(r, theta, phi)
}

// lol
#[allow(dead_code)]
const LOOKUP_X: [f32; 19] = [
    1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216, 0.8962, 0.8679, 0.8350,
    0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322,
];
#[allow(dead_code)]
const LOOKUP_Y: [f32; 19] = [
    0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958, 0.5571, 0.6176, 0.6769,
    0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000,
];

/**
 * input: x0, x1; two floats from the robinson table, and lat: f32, latitude in degrees
 * output: properly interpolated value, idk lol
 */
#[allow(dead_code)]
fn interpolate(x0: f32, x1: f32, lat0: f32, lat: f32) -> f32 {
    x0 + ((lat - lat0) * (x1 - x0)) / 5.0
}

#[allow(dead_code)]
fn robinson_projection(point: Point3<f32>) -> (f32, f32) {
    let idx = (point.y / PI * 18.0) as usize;
    let x0 = LOOKUP_X[idx];
    let x1 = LOOKUP_X[idx + 1];

    let y0 = LOOKUP_Y[idx];
    let y1 = LOOKUP_Y[idx + 1];

    let x = interpolate(x0, x1, idx as f32 * 5.0, point.y / PI * 90.0);
    // let y = y0 + (y1 - y0) * ((idx+1) as f32 - point.y / PI * 18.0);
    let y = interpolate(y0, y1, idx as f32 * 5.0, point.y / PI * 90.0);

    let x = 0.8487 * x * (point.z + PI) / 5.333;
    // let y = point.y / PI;
    (x, y)
}

/**
 * input: Point3<f32>, point in 3d space, spherical coordinates
 * output:
 */
fn projection(point: Point3<f32>) -> (f32, f32) {
    mercator_projection(point)
}

fn mercator_projection(point: Point3<f32>) -> (f32, f32) {
    const LIMIT: f32 = 0.40 * PI;
    let y = point.y - 0.5 * PI;
    let y: f32 = if !(-LIMIT..=LIMIT).contains(&y) {
        y / PI
    } else {
        let len = (PI / 4.0 + LIMIT / 2.0).tan().ln() * 2.0;
        (PI / 4.0 + y / 2.0).tan().ln() / len * 0.80
    } + 0.5;
    let x = point.z / (2.01 * PI) + 0.5;
    (x, y)
}
