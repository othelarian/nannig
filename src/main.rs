extern crate chrono;

use candelabre_core::CandlRenderer;
use candelabre_windowing::{
    CandlDimension, CandlOptions, CandlSurfaceBuilder,
    CandlWindow
};
use chrono::{Duration, Utc};
use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput,
    StartCause, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

use nannig::NannigGraphics;

// HELPERS ====================================================================

fn gap_time() -> std::time::Duration {
    Duration::seconds(1)
        .checked_sub(&Duration::microseconds(Utc::now().timestamp_subsec_micros() as i64))
        .unwrap().to_std().unwrap()
}

// MAIN =======================================================================

fn main() {
    let el = EventLoop::new();

    let options = CandlOptions::default()
        .set_vsync(true)
        //.set_decorations(false)
        .set_transparent(true);

    let mut surface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(800, 400))
        .title("Nannig")
        .options(options)
        .render(NannigGraphics::init())
        .no_state()
        .build(&el)
        .unwrap();

    let mut active_mod = false;
    let mut redraw = false;

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::WaitUntil(Instant::now() + gap_time()),
            Event::NewEvents(StartCause::ResumeTimeReached {..}) => {
                redraw = true;
                *ctrl_flow = ControlFlow::WaitUntil(Instant::now() + gap_time());
            }
            Event::LoopDestroyed => return,
            Event::DeviceEvent {event: DeviceEvent::ModifiersChanged(mod_state), ..} => {
                active_mod =
                    mod_state.shift() || mod_state.ctrl() ||
                    mod_state.alt() || mod_state.logo();
            }
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(psize) => surface.resize(psize),
                WindowEvent::CloseRequested => *ctrl_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => { if !active_mod { *ctrl_flow = ControlFlow::Exit; } }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::A),
                        ..
                    }, ..
                } => {
                    //
                    //
                    //
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                if redraw {
                    //
                    surface.request_redraw();
                    //
                    //
                    redraw = false;
                }
            }
            Event::RedrawRequested(_) => {
                //
                surface.draw();
                //
            }
            _ => ()
        }
    });
}
