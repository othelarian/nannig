use candelabre_core::CandlRenderer;
use candelabre_windowing::{
    CandlDimension, CandlOptions, CandlSurfaceBuilder,
    CandlWindow
};
use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput,
    StartCause, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::{Duration, Instant};

use nannig::NannigGraphics;

// HELPERS ====================================================================

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
    //let start_time = Instant::now();
    let timer_length = Duration::new(1, 0);

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) => *ctrl_flow = ControlFlow::Wait,
            //Event::NewEvents(StartCause::Init) => *ctrl_flow = ControlFlow::WaitUntil(),
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
                //
                //
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
