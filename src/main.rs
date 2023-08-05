use eclat::{Colour, PixelBuffer, Triangle};
use glam::vec2;
use std::num::NonZeroU32;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
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

                pixel_buffer.triangle(Triangle::new(
                    vec2(50., 50.),
                    vec2(140., 60.),
                    vec2(70., 130.),
                ));
                pixel_buffer.triangle(Triangle::new(
                    vec2(140., 60.),
                    vec2(70., 130.),
                    vec2(140., 140.),
                ));

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
