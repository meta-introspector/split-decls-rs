use serde::{Deserialize, Serialize};
use std::collections::HashMap;
extern "system" fn handler(control: u32, ty: u32, data: *mut c_void, context: *mut c_void) -> u32 {
    let service = unsafe { &*(context as *const Service) };
    match control {
        SERVICE_CONTROL_CONTINUE if service.state() == State::Paused => {
            service.set_state(State::ContinuePending);
            service.command(Command::Resume);
            service.set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if service.state() == State::Running => {
            service.set_state(State::PausePending);
            service.command(Command::Pause);
            service.set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            service.set_state(State::StopPending);
            service.command(Command::Stop);
            service.set_state(State::Stopped);
        }
        _ => service.command(Command::Extended(ExtendedCommand { control, ty, data })),
    }
    NO_ERROR
}
