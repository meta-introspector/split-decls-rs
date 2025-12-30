// Generated macro for XSAVE_FORMAT (struct)
macro_rules! Depcrate_b_arch_dependenciesXSAVE_FORMAT {
() => {
// Module: crate::b_arch_dependencies
// Provides: {"XSAVE_FORMAT"}
// Dependencies: {}
# [repr (C)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct XSAVE_FORMAT { pub ControlWord : u16 , pub StatusWord : u16 , pub TagWord : u8 , pub Reserved1 : u8 , pub ErrorOpcode : u16 , pub ErrorOffset : u32 , pub ErrorSelector : u16 , pub Reserved2 : u16 , pub DataOffset : u32 , pub DataSelector : u16 , pub Reserved3 : u16 , pub MxCsr : u32 , pub MxCsr_Mask : u32 , pub FloatRegisters : [M128A ; 8] , pub XmmRegisters : [M128A ; 16] , pub Reserved4 : [u8 ; 96] , }
};
}
