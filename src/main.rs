extern crate kiss3d;
extern crate nalgebra as na; // kinda cool https://www.nalgebra.org/

use kiss3d::camera::ArcBall;
use kiss3d::ncollide3d::math::Point;
use kiss3d::window::Window;
use kiss3d::{event::WindowEvent, light::Light};
use mesh_generation::gen_mesh;
use na::{Point3, Vector3};

mod constants;
mod map;

mod mesh_generation;

fn main() {
    let _karte = map::Map::new("data/earth-heightmap.png");

    let mut window = Window::new("yay");
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point::origin();
    let mut camera = ArcBall::new(eye, at);

    let mesh = gen_mesh(|_point: Point3<f32>| {
        // &karte.height_at_point(point)
        1.0
    });
    let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
    c.set_color(0.8, 0.8, 0.8);
    c.enable_backface_culling(false);

    window.set_light(Light::StickToCamera);

    while !window.should_close() {
        for mut event in window.events().iter() {
            if let WindowEvent::Scroll(_xshift, yshift, _) = event.value {
                let offset = yshift as f32 / 10.0;
                let offset = if offset < -1.0 || offset > 1.0 {
                    offset.signum()
                } else {
                    offset
                };
                camera.set_dist(camera.dist() + offset);
                event.inhibited = true
            }
        }
        window.render_with_camera(&mut camera);
    }

    // read_png("data/earth-heightmap.png");
}
