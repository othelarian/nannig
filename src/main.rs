extern crate chrono;

use candelabre_windowing::{
    CandlManager, CandlWindow
};
use chrono::{Duration, Utc};
use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput,
    StartCause, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

use nannig::{NannigMessage, NannigStore, nannig_wins};

// HELPERS ====================================================================

fn gap_time() -> std::time::Duration {
    Duration::seconds(1)
        .checked_sub(&Duration::microseconds(Utc::now().timestamp_subsec_micros() as i64))
        .unwrap().to_std().unwrap()
}

// MAIN =======================================================================

fn main() {
    let el = EventLoop::new();

    let mut manager = CandlManager::new();

    let _classic = manager
        .create_window_from_builder(nannig_wins::classic_win(), &el)
        .unwrap();
    let mut store = NannigStore::new();

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::WaitUntil(Instant::now() + gap_time()),
            Event::NewEvents(StartCause::ResumeTimeReached {..}) => {
                //
                //store.need_redraw();
                //
                for wid in manager.list_window_ids() {
                    let surface = manager.get_current(wid).unwrap();
                    let state = surface.state_mut();
                    state.need_redraw();
                }
                //
                // TODO : ask classic and fullscreen windows to redraw
                //
                *ctrl_flow = ControlFlow::WaitUntil(Instant::now() + gap_time());
            }
            Event::LoopDestroyed => return,
            Event::DeviceEvent {
                event: DeviceEvent::ModifiersChanged(mod_state), ..
            } => store.update_mods(mod_state),
            Event::WindowEvent {event, window_id} => match event {
                WindowEvent::Resized(psize) =>
                    manager.get_current(window_id).unwrap().resize(psize),
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(keycode),
                        ..
                    }, ..
                } => {
                    match store.handle_keycode(keycode) {
                        NannigMessage::Config => {
                            //
                            //
                        }
                        NannigMessage::Nothing => (),
                        NannigMessage::Quit => *ctrl_flow = ControlFlow::Exit
                    }
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                for wid in manager.list_window_ids() {
                    let surface = manager.get_current(wid).unwrap();
                    if surface.state_mut().redraw_asked() {
                        surface.request_redraw();
                    }
                }
            }
            Event::RedrawRequested(win_id) => {
                manager.get_current(win_id).unwrap().draw();
            }
            _ => ()
        }
    });
}
