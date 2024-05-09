use std::num::NonZeroU32;
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
    let mut cells: [u8; CELLS_COUNT] = [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        // println!("Event {:?}", event);

        match event {
            Event::NewEvents(start_clause) if start_clause == StartCause::Poll => {
                let (screen_width, screen_height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                cells = update_cells(cells);
                surface
                    .resize(
                        NonZeroU32::new(screen_width).unwrap(),
                        NonZeroU32::new(screen_height).unwrap()
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                let pixels = (screen_width as usize) * (screen_height as usize);
                let adjustment_factor = pixels / CELLS_COUNT;
                for ii in 0..pixels {
                    let mut relative_index = ii / adjustment_factor;
                    if relative_index > CELLS_COUNT - 1 {
                        relative_index = CELLS_COUNT - 1;
                    }
                    buffer[ii] = if cells[relative_index] == 0 { WHITE } else { BLACK };
                }

                buffer.present().unwrap();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (screen_width, screen_height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                cells = update_cells(cells);
                surface
                    .resize(
                        NonZeroU32::new(screen_width).unwrap(),
                        NonZeroU32::new(screen_height).unwrap()
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                let pixels = (screen_width as usize) * (screen_height as usize);
                let adjustment_factor = pixels / CELLS_COUNT;
                for ii in 0..pixels {
                    let mut relative_index = ii / adjustment_factor;
                    if relative_index > CELLS_COUNT - 1 {
                        relative_index = CELLS_COUNT - 1;
                    }
                    buffer[ii] = if cells[relative_index] == 0 { WHITE } else { BLACK };
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, window_id } if
                window_id == window.id()
            => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
