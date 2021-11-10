use std::{cell::RefCell, rc::Rc};

use crate::constants::{LEN, RADIUS};
use kiss3d::resource::Mesh;
use nalgebra::Point3;

trait Normalize {
    fn normalize(&self) -> Point3<f32>;
    fn scalar_mult(&self, scalar: f32) -> Point3<f32>;
}

impl Normalize for Point3<f32> {
    fn normalize(&self) -> Point3<f32> {
        // let abs: f32 = self.x * self.x + self.y * self.y + self.z * self.z;
        // let abs = abs.sqrt() / 10.;
        // Point3::new(self.x / abs, self.y / abs, self.z / abs)

        let x2: f32 = self.x * self.x;
        let y2: f32 = self.y * self.y;
        let z2: f32 = self.z * self.z;

        // let x = self.x / (x2+y2+z2).sqrt();
        // let y = self.y / (x2+y2+z2).sqrt();
        // let z = self.z / (x2+y2+z2).sqrt();
        let x: f32 = self.x * ((1.0 - (y2 + z2) / 2.0 + (y2 * z2) / 3.).sqrt());
        let y: f32 = self.y * ((1.0 - (z2 + x2) / 2.0 + (x2 * z2) / 3.).sqrt());
        let z: f32 = self.z * ((1.0 - (x2 + y2) / 2.0 + (x2 * y2) / 3.).sqrt());
        // println!("({}, {}, {}) -> {}, {}, {}",self.x, self.y, self.z, x,y,z);
        Point3::new(x, y, z)
    }

    fn scalar_mult(&self, s: f32) -> Point3<f32> {
        Point3::new(self.x * s, self.y * s, self.z * s)
    }
}

fn make_point<F: Fn(Point3<f32>) -> f32>(i: f32, j: f32, side: u8, height: &F) -> Point3<f32> {
    let out = match side {
        0 => Point3::new(i, j, RADIUS),  // front
        1 => Point3::new(i, j, -RADIUS), // back
        2 => Point3::new(i, RADIUS, j),
        3 => Point3::new(i, -RADIUS, j),
        4 => Point3::new(RADIUS, i, j),
        5 => Point3::new(-RADIUS, i, j),
        _ => unreachable!(),
    }
    .normalize();
    let height = height(out);
    out.scalar_mult(height)
}

pub fn gen_mesh<F: Fn(Point3<f32>) -> f32>(height: F) -> Rc<RefCell<Mesh>> {
    let mut vertices = vec![];
    let mut indices = vec![];
    let c = (2. * RADIUS) / (LEN as f32);
    let mut index = 0;
    for side in 0..6u8 {
        for i in 0..LEN {
            for j in 0..LEN {
                let x: f32 = j as f32 * c - 1.;
                let y: f32 = i as f32 * c - 1.;
                // not optimal
                // redeclare like most of the vertices but it's fine
                vertices.push(make_point(x, y, side, &height));
                vertices.push(make_point(x + c, y, side, &height));
                vertices.push(make_point(x, y + c, side, &height));
                vertices.push(make_point(x + c, y + c, side, &height));
                indices.push(Point3::new(
                    index as u16,
                    (index + 1) as u16,
                    (index + 2) as u16,
                ));
                indices.push(Point3::new(
                    (index + 1) as u16,
                    (index + 2) as u16,
                    (index + 3) as u16,
                ));
                index += 4;
            }
        }
    }
    // println!("{:?} {:?}", indices.len(), vertices.len());

    Rc::new(RefCell::new(Mesh::new(
        vertices, indices, None, None, false,
    )))
}