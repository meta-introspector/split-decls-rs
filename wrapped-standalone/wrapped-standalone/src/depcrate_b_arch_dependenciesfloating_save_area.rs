// Generated macro for FLOATING_SAVE_AREA (struct)
macro_rules! Depcrate_b_arch_dependenciesFLOATING_SAVE_AREA {
() => {
// Module: crate::b_arch_dependencies
// Provides: {"FLOATING_SAVE_AREA"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct FLOATING_SAVE_AREA { pub ControlWord : u32 , pub StatusWord : u32 , pub TagWord : u32 , pub ErrorOffset : u32 , pub ErrorSelector : u32 , pub DataOffset : u32 , pub DataSelector : u32 , pub RegisterArea : [u8 ; 80] , pub Cr0NpxState : u32 , }
};
}
