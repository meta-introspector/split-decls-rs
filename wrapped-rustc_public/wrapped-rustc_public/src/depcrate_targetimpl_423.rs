// Generated macro for impl_423 (impl)
macro_rules! Depcrate_targetimpl_423 {
() => {
// Module: crate::target
// Provides: {"impl_423"}
// Dependencies: {}
impl MachineInfo { pub fn target () -> MachineInfo { with (| cx | cx . target_info ()) } pub fn target_endianness () -> Endian { with (| cx | cx . target_info () . endian) } pub fn target_pointer_width () -> MachineSize { with (| cx | cx . target_info () . pointer_width) } }
};
}
