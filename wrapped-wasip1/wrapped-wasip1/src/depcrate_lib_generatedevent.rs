// Generated macro for Event (struct)
macro_rules! Depcrate_lib_generatedEvent {
() => {
// Module: crate::lib_generated
// Provides: {"Event"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct Event { # [doc = " User-provided value that got attached to `subscription::userdata`."] pub userdata : Userdata , # [doc = " If non-zero, an error that occurred while processing the subscription request."] pub error : Errno , # [doc = " The type of event that occured"] pub type_ : Eventtype , # [doc = " The contents of the event, if it is an `eventtype::fd_read` or"] # [doc = " `eventtype::fd_write`. `eventtype::clock` events ignore this field."] pub fd_readwrite : EventFdReadwrite , }
};
}
