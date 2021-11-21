extern crate kiss3d;
extern crate nalgebra as na; // kinda cool https://www.nalgebra.org/

use kiss3d::light::Light;
use kiss3d::ncollide3d::math::Point;
use kiss3d::window::Window;
use kiss3d::{camera::ArcBall, event::WindowEvent};
use mesh_generation::gen_mesh;
use na::{Point3, Vector3};

mod constants;
mod map;

mod mesh_generation;

fn main() {
    let karte = map::Map::new("data/earth-heightmap.png");

    let mut window = Window::new("yay");
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point::origin();
    let mut camera = ArcBall::new(eye, at);

    let mut faces = vec![];
    for side in 0..6u8 {
        let mesh = gen_mesh(|p: Point3<f32>| karte.height_at(p), side);
        let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
        
        c.set_color(0.6, 0.6,  0.8);
        c.enable_backface_culling(false);
        faces.push(c);
    }

    window.set_light(Light::StickToCamera);
    let mut is_wireframe = false;

    while !window.should_close() {
        for mut event in window.events().iter() {
            if let WindowEvent::Scroll(_xshift, yshift, _) = event.value {
                let offset = yshift as f32 / 10.0;
                let offset = if !(-0.1..=0.1).contains(&offset) {
                    offset.signum()
                } else {
                    offset
                } / 10.0;
                if camera.dist() + offset > 1.2 {
                    camera.set_dist(camera.dist() + offset);
                } else {
                    camera.set_dist(1.20);
                }
                event.inhibited = true
            } else if let WindowEvent::Char(ch) = event.value {
                if ch == 'w' {
                    for face in &mut faces {
                    if is_wireframe {
                            face.set_points_size(2.0);
                            face.set_lines_width(1.0);
                            face.set_surface_rendering_activation(false);
                    } else {
                            face.set_points_size(0.0);
                            face.set_lines_width(0.0);
                            face.set_surface_rendering_activation(true);
                        }
                    }
                    is_wireframe = !is_wireframe;
                }
            }
        }
        window.render_with_camera(&mut camera);
    }
}
