// Generated macro for MarkedEventReceiver (trait)
macro_rules! Depcrate_parserMarkedEventReceiver {
() => {
// Module: crate::parser
// Provides: {"MarkedEventReceiver"}
// Dependencies: {}
# [doc = " Trait to be implemented for using the low-level parsing API."] # [doc = ""] # [doc = " Functionally similar to [`EventReceiver`], but receives a [`Marker`] as well as the event."] pub trait MarkedEventReceiver { # [doc = " Handler called for each event that occurs."] fn on_event (& mut self , ev : Event , _mark : Marker) ; }
};
}
