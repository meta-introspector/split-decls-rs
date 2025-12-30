// Generated macro for Controller (struct)
macro_rules! Depcrate_controllerController {
() => {
// Module: crate::controller
// Provides: {"Controller"}
// Dependencies: {}
# [doc = " The controller of the application turns page state into functionality"] pub struct Controller { store : Store , sched : RefCell < Option < Weak < Scheduler > > > , active_route : String , last_active_route : String , }
};
}
