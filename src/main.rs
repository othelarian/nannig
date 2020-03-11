extern crate chrono;

use candelabre_windowing::{
    CandlManager, CandlWindow
};
use chrono::{Duration, Utc};
use glutin::event::{
    DeviceEvent, ElementState, Event, KeyboardInput,
    StartCause, VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

use nannig::{NannigMessage, NannigStore, nannig_config, nannig_wins};
use nannig_config::NannigCfg;
use nannig_wins::NannigWinType;

// HELPERS ====================================================================

fn gap_time() -> std::time::Duration {
    Duration::seconds(1)
        .checked_sub(&Duration::microseconds(Utc::now().timestamp_subsec_micros() as i64))
        .unwrap().to_std().unwrap()
}

// MAIN =======================================================================

fn main() {
    //
    // TODO : check config existence
    //
    NannigCfg::check_conf_file();
    //
    //
    // TODO : generate NannigCfg
    //

    let el = EventLoop::new();
    let mut manager = CandlManager::new();

    let video_mode = el.primary_monitor().video_modes().next().unwrap();
    let classic_id = manager
        .create_window_from_builder(nannig_wins::classic_win().video_mode(video_mode), &el)
        .unwrap();

    let mut store = NannigStore::new();
    store.set_classic_win(Some(classic_id));

    el.run(move |evt, el_wt, ctrl_flow| {
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
                WindowEvent::CloseRequested => {
                    match manager.get_current(window_id).unwrap().state().get_type() {
                        NannigWinType::Classic => *ctrl_flow = ControlFlow::Exit,
                        NannigWinType::Config => {
                            manager.remove_window(window_id);
                            store.set_config_win(None);
                        }
                        _ => ()
                    };
                }
                WindowEvent::Resized(psize) =>
                    manager.get_current(window_id).unwrap().resize(psize),
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => {
                    let config_id = store.get_config_win();
                    if config_id.is_some() && config_id.unwrap() == window_id {
                        manager.remove_window(window_id);
                        store.set_config_win(None);
                    }
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(keycode),
                        ..
                    }, ..
                } => {
                    match store.handle_keycode(keycode) {
                        NannigMessage::Classic => {
                            let f_wins = store.get_fullscreen_wins();
                            let video_mode = manager
                                .get_current(f_wins.first().unwrap().clone())
                                .unwrap()
                                .get_window()
                                .unwrap()
                                .primary_monitor()
                                .video_modes()
                                .next()
                                .unwrap();
                            for w_id in f_wins { manager.remove_window(w_id.clone()); }
                            store.clear_fullscreen_wins();
                            let classic_id = manager.create_window_from_builder(
                                nannig_wins::classic_win().video_mode(video_mode),
                                el_wt
                            ).unwrap();
                            store.set_classic_win(Some(classic_id));
                        }
                        NannigMessage::ConfigClose => {
                            manager.remove_window(store.get_config_win().unwrap());
                            store.set_config_win(None);
                        }
                        NannigMessage::ConfigOpen => {
                            let classic_id = store.get_classic_win().unwrap();
                            let classic_surface = manager.get_current(classic_id).unwrap();
                            let video_mode = classic_surface
                                .get_window().unwrap()
                                .primary_monitor()
                                .video_modes()
                                .next().unwrap();
                            let config_id = manager
                                .create_window_from_builder(
                                    nannig_wins::config_win().video_mode(video_mode),
                                    el_wt
                                )
                                .unwrap();
                            store.set_config_win(Some(config_id));
                        }
                        NannigMessage::Fullscreen => {
                            if let Some(config_id) = store.get_config_win() {
                                manager.remove_window(config_id);
                                store.set_config_win(None);
                            }
                            let classic_id = store.get_classic_win().unwrap();
                            let monitors = manager.get_current(classic_id)
                                .unwrap()
                                .get_window()
                                .unwrap()
                                .available_monitors();
                            manager.remove_window(classic_id);
                            store.set_classic_win(None);
                            for monitor in monitors {
                                let builder = nannig_wins::fullscreen_win()
                                    .video_mode(monitor.video_modes().next().unwrap());
                                let w_id = manager.create_window_from_builder(builder, el_wt).unwrap();
                                store.add_fullscreen_win(w_id);
                            }
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
