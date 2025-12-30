// Generated macro for State (enum)
macro_rules! Depcrate_thread_atomics_oneshotState {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"State"}
// Dependencies: {}
# [doc = " Current state of the value."] enum State < T > { # [doc = " Waiting for a value to be delivered."] Waiting , # [doc = " [`Sender`] dropped."] Dropped , # [doc = " Value taken."] Taken , # [doc = " Value arrived."] Result (T) , }
};
}
