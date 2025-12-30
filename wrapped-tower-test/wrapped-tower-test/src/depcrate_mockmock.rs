// Generated macro for Mock (struct)
macro_rules! Depcrate_mockMock {
() => {
// Module: crate::mock
// Provides: {"Mock"}
// Dependencies: {}
# [doc = " A mock service"] # [derive (Debug)] pub struct Mock < T , U > { id : u64 , tx : Mutex < Tx < T , U > > , state : Arc < Mutex < State > > , can_send : bool , }
};
}
