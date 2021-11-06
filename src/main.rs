extern crate kiss3d;
extern crate nalgebra as na; // kinda cool https://www.nalgebra.org/

use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, Vector3};

const LEN: i32 = 50;
const RADIUS: f32 = 1.0f32;
mod map;

trait Normaliz {
    fn normalize(&self) -> Point3<f32>;
    fn scalar_mult(&self, scalar: f32) -> Point3<f32>;
}

impl Normaliz for Point3<f32> {
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

fn make_point(i: f32, j: f32, side: u8, karte: &map::Map) -> Point3<f32> {
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
    //let height = karte.height_at_point(out);
    out //.scalar_mult(height)
}

fn gen_mesh(karte: &map::Map) -> Rc<RefCell<Mesh>> {
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
                vertices.push(make_point(x, y, side, karte));
                vertices.push(make_point(x + c, y, side, karte));
                vertices.push(make_point(x, y + c, side, karte));
                vertices.push(make_point(x + c, y + c, side, karte));
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

fn main() {
    let karte = map::Map::new("data/earth-heightmap.png");

    let mut window = Window::new("yay");
    let mesh = gen_mesh(&karte);

    // // let mesh = Rc::new(RefCell::new(Mesh::new(
    // //     vert, indices, None, None, false
    // // )));

    let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
    c.set_color(0.8, 0.8, 0.8);
    c.enable_backface_culling(false);

    window.set_light(Light::StickToCamera);

    while window.render() {
        // c.prepend_to_local_rotation(&rot);
    }

    // read_png("data/earth-heightmap.png");
}
