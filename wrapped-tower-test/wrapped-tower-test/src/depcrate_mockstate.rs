// Generated macro for State (struct)
macro_rules! Depcrate_mockState {
() => {
// Module: crate::mock
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] struct State { # [doc = " Tracks the number of requests that can be sent through"] rem : u64 , # [doc = " Tasks that are blocked"] tasks : HashMap < u64 , Waker > , # [doc = " Tracks if the `Handle` dropped"] is_closed : bool , # [doc = " Tracks the ID for the next mock clone"] next_clone_id : u64 , # [doc = " Tracks the next error to yield (if any)"] err_with : Option < Error > , }
};
}
