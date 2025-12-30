// Generated macro for spawn (function)
macro_rules! Depcrate_mockspawn {
() => {
// Module: crate::mock
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " Spawn a Service onto a mock task."] pub fn spawn < T , U > () -> (Spawn < Mock < T , U > > , Handle < T , U >) { let (svc , handle) = pair () ; (Spawn :: new (svc) , handle) }
};
}
