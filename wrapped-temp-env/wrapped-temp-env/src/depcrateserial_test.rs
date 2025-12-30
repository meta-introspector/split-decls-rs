// Generated macro for SERIAL_TEST (static)
macro_rules! DepcrateSERIAL_TEST {
() => {
// Module: crate
// Provides: {"SERIAL_TEST"}
// Dependencies: {}
# [doc = " Make sure that the environment isn't modified concurrently."] static SERIAL_TEST : ReentrantMutex < () > = ReentrantMutex :: new (()) ;
};
}
