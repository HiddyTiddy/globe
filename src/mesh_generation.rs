use std::{cell::RefCell, rc::Rc};

use crate::constants::{LEN, RADIUS};
use kiss3d::resource::Mesh;
use nalgebra::Point3;
// use kiss3d::nalgebra::OPoint;

/*
dumb trait for Point3<f32>
*/
trait Normalize {
    fn normalize(&self) -> Self;
    fn scalar_mult(&self, scalar: f32) -> Self;
}

impl Normalize for Point3<f32> {
    fn normalize(&self) -> Point3<f32> {
        // weird normalize function that i yanked
        let x2: f32 = self.x * self.x;
        let y2: f32 = self.y * self.y;
        let z2: f32 = self.z * self.z;

        // let x = self.x / (x2+y2+z2).sqrt();
        // let y = self.y / (x2+y2+z2).sqrt();
        // let z = self.z / (x2+y2+z2).sqrt();
        let x: f32 = self.x * ((1.0 - (y2 + z2) / 2.0 + (y2 * z2) / 3.).sqrt());
        let y: f32 = self.y * ((1.0 - (z2 + x2) / 2.0 + (x2 * z2) / 3.).sqrt());
        let z: f32 = self.z * ((1.0 - (x2 + y2) / 2.0 + (x2 * y2) / 3.).sqrt());
        Point3::new(x, y, z)
    }

    fn scalar_mult(&self, s: f32) -> Point3<f32> {
        Point3::new(self.x * s, self.y * s, self.z * s)
    }
}

fn make_point<F>(i: f32, j: f32, side: u8, height: &F) -> Point3<f32>
where
    F: Fn(Point3<f32>) -> f32,
{
    // i like where syntax
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
    let h = height(out);
    out.scalar_mult(h)
}

/// generate mesh function
pub fn gen_mesh<F: Fn(Point3<f32>) -> f32>(height: F, side: u8) -> Rc<RefCell<Mesh>> {
    /*!
     * this function should, given a function that calculates at a given point on the unit sphere, return a mesh of the globe
     */
    let mut vertices = vec![];
    let mut indices = vec![];
    let c = (2. * RADIUS) / (LEN as f32);
    let mut index: u16 = 0;
    for j in 0..LEN {
        let x: f32 = j as f32 * c - 1.0;
        let y: f32 = -1.0;
        vertices.push(make_point(x, y, side, &height));
        vertices.push(make_point(x + c, y, side, &height));
        index += 2;
    }

    for i in 0..LEN {
        for j in 0..LEN {
            let x: f32 = j as f32 * c - 1.;
            let y: f32 = i as f32 * c - 1.;
            // better
            vertices.push(make_point(x, y + c, side, &height));
            vertices.push(make_point(x + c, y + c, side, &height));
            // side == 1 | 2 | 5
            if side == 1 || side == 2 || side == 5 {
                indices.push(Point3::new(index, index + 1, index - (2 * LEN) as u16 + 1));
                indices.push(Point3::new(
                    index,
                    index - (2 * LEN) as u16 + 1,
                    index - (2 * LEN) as u16,
                ));
            } else {
                indices.push(Point3::new(index - (2 * LEN) as u16 + 1, index + 1, index));
                indices.push(Point3::new(
                    index - (2 * LEN) as u16,
                    index - (2 * LEN) as u16 + 1,
                    index,
                ));
            }
            index += 2;
        }
    }
    // println!("{:?} {:?}", indices.len(), vertices.len());
    Rc::new(RefCell::new(Mesh::new(
        vertices, indices, None, None, false,
    )))
}
