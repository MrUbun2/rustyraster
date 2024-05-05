use crate::types::*;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::EventPump;

fn camera_transform(geometry: &mut Vec<Triangle>, camera: &Camera) {
    let pos = camera.pos.clone();
    let rot = camera.rot.clone();

    let rx = -rot.x.to_radians();
    let ry = -rot.y.to_radians();
    let rz = -rot.z.to_radians();

    let sx = rx.sin();
    let cx = rx.cos();
    let sy = ry.sin();
    let cy = ry.cos();
    let sz = rz.sin();
    let cz = rz.cos();

    for triangle in geometry {
        for vertex in &mut triangle.v {
            let x = vertex.x - pos.x;
            let y = vertex.y - pos.y;
            let z = vertex.z - pos.z;
            vertex.x = cy * (sz * y + cz * x) - sy * z;
            vertex.y = sx * (cy * z + sy * (sz * y + cz * x)) + cx * (cz * y - sz * x);
            vertex.z = cx * (cy * z + sy * (sz * y + cz * x)) - sx * (cz * y - sz * x);
        }
    }
}

pub fn render(
    window: &mut Window,
    event_pump: &EventPump,
    geometry: &Vec<Triangle>,
    camera: &Camera,
) -> Result<(), String> {
    let mut surface = window.surface(event_pump)?;
    let mut newgeo = geometry.clone();
    camera_transform(&mut newgeo, camera);

    let _ = surface.fill_rect(Rect::new(5, 5, 50, 50), (100, 10, 40).into());

    surface.finish()
}
