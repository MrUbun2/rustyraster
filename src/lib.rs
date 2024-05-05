pub mod engine;
pub mod types;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::types::*;
    use super::*;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn render_test() -> Result<(), String> {
        let width = 500u32;
        let height = 500u32;
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut window = video_subsystem
            .window("rustyraster test", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let mut event_pump = sdl_context.event_pump()?;

        let v1 = Vec3f::new(0.0, 0.0, 1.0);
        let v2 = Vec3f::new(1.0, 0.0, 1.0);
        let v3 = Vec3f::new(0.0, 1.0, 1.0);

        let triangle = Triangle { v: [v1, v2, v3] };

        let geometry = vec![triangle];

        let camera = Camera {
            pos: Vec3f::new(0.0, 0.0, 5.0),
            rot: Vec3f::new(0.0, 0.0, 0.0),
            fov: 50.0,
        };

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            engine::render(&mut window, &event_pump, &geometry, &camera)?;

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(()) // ok :thumbsup:
    }
}
