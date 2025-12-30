// Generated macro for macro_1365 (macro)
macro_rules! Depcrate_shared_bthioctlmacro_1365 {
() => {
// Module: crate::shared::bthioctl
// Provides: {"macro_1365"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct BTH_VENDOR_SPECIFIC_COMMAND { ManufacturerId : ULONG , LmpVersion : UCHAR , MatchAnySinglePattern : BOOLEAN , HciHeader : BTH_COMMAND_HEADER , Data : [UCHAR ; 1] , } }
};
}
