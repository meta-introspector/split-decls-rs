// Generated macro for park_timeout_ms (function)
macro_rules! Depcrate_threadpark_timeout_ms {
() => {
// Module: crate::thread
// Provides: {"park_timeout_ms"}
// Dependencies: {}
# [doc = " Uses [`park_timeout`]."] # [doc = ""] # [doc = " Blocks unless or until the current thread's token is made available or"] # [doc = " the specified duration has been reached (may wake spuriously)."] # [doc = ""] # [doc = " The semantics of this function are equivalent to [`park`] except"] # [doc = " that the thread will be blocked for roughly no longer than `dur`. This"] # [doc = " method should not be used for precise timing due to anomalies such as"] # [doc = " preemption or platform differences that might not cause the maximum"] # [doc = " amount of time waited to be precisely `ms` long."] # [doc = ""] # [doc = " See the [park documentation][`park`] for more detail."] # [stable (feature = "rust1" , since = "1.0.0")] # [deprecated (since = "1.6.0" , note = "replaced by `std::thread::park_timeout`")] pub fn park_timeout_ms (ms : u32) { park_timeout (Duration :: from_millis (ms as u64)) }
};
}
