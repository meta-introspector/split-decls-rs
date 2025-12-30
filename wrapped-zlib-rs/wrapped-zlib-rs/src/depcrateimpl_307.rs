// Generated macro for impl_307 (impl)
macro_rules! Depcrateimpl_307 {
() => {
// Module: crate
// Provides: {"impl_307"}
// Dependencies: {}
impl TryFrom < i32 > for InflateFlush { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 0 => Ok (Self :: NoFlush) , 2 => Ok (Self :: SyncFlush) , 4 => Ok (Self :: Finish) , 5 => Ok (Self :: Block) , 6 => Ok (Self :: Trees) , _ => Err (()) , } } }
};
}
