// Generated macro for app (function)
macro_rules! Depcrateapp {
() => {
// Module: crate
// Provides: {"app"}
// Dependencies: {}
fn app (name : & str) { let sched = Rc :: new (Scheduler :: new ()) ; let store = match Store :: new (name) { Some (s) => s , None => return , } ; let controller = Controller :: new (store , Rc :: downgrade (& sched)) ; if let Some (mut view) = View :: new (Rc :: clone (& sched)) { let sch : & Rc < Scheduler > = & sched ; view . init () ; sch . set_view (view) ; sch . set_controller (controller) ; sched . add_message (Message :: Controller (ControllerMessage :: SetPage ("" . to_string () ,))) ; } }
};
}
