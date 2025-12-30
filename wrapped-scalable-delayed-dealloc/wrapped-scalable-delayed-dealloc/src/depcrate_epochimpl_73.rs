// Generated macro for impl_73 (impl)
macro_rules! Depcrate_epochimpl_73 {
() => {
// Module: crate::epoch
// Provides: {"impl_73"}
// Dependencies: {}
impl TryFrom < u8 > for Epoch { type Error = Epoch ; # [inline] fn try_from (value : u8) -> Result < Self , Self :: Error > { if value < Self :: NUM_EPOCHS { Ok (Epoch { value }) } else { Err (Epoch { value : value % Self :: NUM_EPOCHS , }) } } }
};
}
