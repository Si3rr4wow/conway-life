use std::num::NonZeroU32;
use std::cmp::min;
use rand::Rng;
use winit::dpi::LogicalSize;
use winit::event::{ Event, StartCause, WindowEvent };
use winit::event_loop::{ ControlFlow, EventLoop };
use winit::window::WindowBuilder;

use crate::cells::update_cells;
use crate::{ H, W };
const CELLS_COUNT: usize = H * W;
const BLACK: u32 = 0;
const WHITE: u32 = 255 + (255 << 8) + (255 << 16);

pub fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let context = (unsafe { softbuffer::Context::new(&window) }).unwrap();
    let mut surface = (unsafe { softbuffer::Surface::new(&context, &window) }).unwrap();
    let mut cells: [u8; CELLS_COUNT] = [0; CELLS_COUNT];

    window.set_inner_size(LogicalSize::new(256, 256));

    let mut rng = rand::thread_rng();
    for ii in 0..CELLS_COUNT {
        let value: u8 = rng.gen::<u8>() / 180;
        cells[ii] = value;
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let (screen_width, screen_height) = {
            let size = window.inner_size();
            (size.width, size.height)
        };
        let mut width_scale_factor = (screen_width as usize) / W;
        let mut height_scale_factor = (screen_height as usize) / H;

        match event {
            Event::NewEvents(start_clause) if start_clause == StartCause::Poll => {
                cells = update_cells(cells);
                surface
                    .resize(
                        NonZeroU32::new(screen_width).unwrap(),
                        NonZeroU32::new(screen_height).unwrap()
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                for ii in 0..(screen_width as usize) * (screen_height as usize) {
                    let x = ii % (screen_width as usize);
                    let y = ii / (screen_width as usize);

                    let relative_x_index = x / width_scale_factor;
                    let relative_y_index = y / height_scale_factor;
                    let relative_index = min(
                        CELLS_COUNT - 1,
                        relative_x_index + relative_y_index * W
                    );
                    let color = {
                        if cells[relative_index] == 0 { WHITE } else { BLACK }
                    };
                    buffer[ii] = color;
                }
                buffer.present().unwrap();
            }

            Event::RedrawRequested(window_id) if window_id == window.id() => {
                surface
                    .resize(
                        NonZeroU32::new(screen_width).unwrap(),
                        NonZeroU32::new(screen_height).unwrap()
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                let pixels: usize = (screen_width as usize) * (screen_height as usize);
                for ii in 0..pixels {
                    let x = ii % (screen_width as usize);
                    let y = ii / (screen_width as usize);

                    let relative_x_index = x / width_scale_factor;
                    let relative_y_index = y / height_scale_factor;
                    let relative_index = min(
                        CELLS_COUNT - 1,
                        relative_x_index + relative_y_index * W
                    );
                    let color = {
                        if cells[relative_index] == 0 { WHITE } else { BLACK }
                    };
                    buffer[ii] = color;
                }
                buffer.present().unwrap();
            }

            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(_) => {
                        let (screen_width, screen_height) = {
                            let size = window.inner_size();
                            (size.width, size.height)
                        };
                        width_scale_factor = (screen_width as usize) / W;
                        height_scale_factor = (screen_height as usize) / H;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {} // this is a neat lil guy to avoid writing every match
                }
            }
            _ => {}
        }
    });
}
