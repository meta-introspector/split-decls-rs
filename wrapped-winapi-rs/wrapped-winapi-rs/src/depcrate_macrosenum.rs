// Generated macro for ENUM (macro)
macro_rules! Depcrate_macrosENUM {
() => {
// Module: crate::macros
// Provides: {"ENUM"}
// Dependencies: {}
# [macro_export] macro_rules ! ENUM { { enum $ name : ident { $ ($ variant : ident = $ value : expr ,) + } } => { pub type $ name = u32 ; $ (pub const $ variant : $ name = $ value ;) + } ; { enum $ name : ident { $ variant : ident = $ value : expr , $ ($ rest : tt) * } } => { pub type $ name = u32 ; pub const $ variant : $ name = $ value ; ENUM ! { @ gen $ name $ variant , $ ($ rest) * } } ; { enum $ name : ident { $ variant : ident , $ ($ rest : tt) * } } => { ENUM ! { enum $ name { $ variant = 0 , $ ($ rest) * } } } ; { @ gen $ name : ident $ base : ident , } => { } ; { @ gen $ name : ident $ base : ident , $ variant : ident = $ value : expr , $ ($ rest : tt) * } => { pub const $ variant : $ name = $ value ; ENUM ! { @ gen $ name $ variant , $ ($ rest) * } } ; { @ gen $ name : ident $ base : ident , $ variant : ident , $ ($ rest : tt) * } => { pub const $ variant : $ name = $ base + 1u32 ; ENUM ! { @ gen $ name $ variant , $ ($ rest) * } } ; }
};
}
