// Generated macro for IsMITMProtectionRequired (function)
macro_rules! Depcrate_shared_bthdefIsMITMProtectionRequired {
() => {
// Module: crate::shared::bthdef
// Provides: {"IsMITMProtectionRequired"}
// Dependencies: {}
# [inline] pub fn IsMITMProtectionRequired (requirements : AUTHENTICATION_REQUIREMENTS) -> bool { MITMProtectionRequired == requirements || MITMProtectionRequiredBonding == requirements || MITMProtectionRequiredGeneralBonding == requirements }
};
}
