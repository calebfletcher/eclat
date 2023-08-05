use eclat::colour::Colour;
use eclat::mesh::Mesh;
use eclat::PixelBuffer;
use glam::vec3;
use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("eclat")
        .build(&event_loop)
        .unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                let mut pixel_buffer =
                    PixelBuffer::new(&mut buffer, width as usize, height as usize);
                pixel_buffer.clear(Colour::BLACK);

                let vertices = vec![
                    vec3(-0.9, -0.9, 0.),
                    vec3(0.9, -0.9, 0.),
                    vec3(0.9, 0.9, 0.),
                    vec3(0.9, 0.9, 0.),
                    vec3(-0.9, 0.9, 0.),
                    vec3(-0.9, -0.9, 0.),
                ];
                let indices = vec![0, 1, 2, 3, 4, 5];
                let colours = [Colour::RED, Colour::GREEN, Colour::BLUE]
                    .into_iter()
                    .cycle()
                    .take(vertices.len())
                    .collect();

                let mesh = Mesh::new(vertices, indices, colours);
                pixel_buffer.mesh(mesh);

                pixel_buffer.save_buffer("out.png");

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
