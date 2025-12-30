// Generated macro for restricted_for_arm64ec (function)
macro_rules! Depcrate_asm_aarch64restricted_for_arm64ec {
() => {
// Module: crate::asm::aarch64
// Provides: {"restricted_for_arm64ec"}
// Dependencies: {}
fn restricted_for_arm64ec (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if arch == InlineAsmArch :: Arm64EC { Err ("x13, x14, x23, x24, x28, v16-v31, p*, ffr cannot be used for Arm64EC") } else { Ok (()) } }
};
}
