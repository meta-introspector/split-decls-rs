// Generated macro for Scheduler (struct)
macro_rules! Depcrate_schedulerScheduler {
() => {
// Module: crate::scheduler
// Provides: {"Scheduler"}
// Dependencies: {}
# [doc = " Creates an event loop that starts each time a message is added"] pub struct Scheduler { controller : Rc < RefCell < Option < Controller > > > , view : Rc < RefCell < Option < View > > > , events : RefCell < Vec < Message > > , running : RefCell < bool > , }
};
}
