// Generated macro for Tx (type)
macro_rules! Depcrate_buffer_messageTx {
() => {
// Module: crate::buffer::message
// Provides: {"Tx"}
// Dependencies: {}
# [doc = " Response sender"] pub (crate) type Tx < Fut > = oneshot :: Sender < Result < Fut , ServiceError > > ;
};
}
