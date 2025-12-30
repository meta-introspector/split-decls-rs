// Generated macro for char (module)
macro_rules! Depcratechar {
() => {
// Module: crate
// Provides: {"char"}
// Dependencies: {}
# [doc = " Methods for composing and decomposing characters."] pub mod char { pub use crate :: normalize :: { compose , decompose_canonical , decompose_cjk_compat_variants , decompose_compatible , } ; pub use crate :: lookups :: { canonical_combining_class , is_combining_mark } ; # [doc = " Return whether the given character is assigned (`General_Category` != `Unassigned`)"] # [doc = " and not Private-Use (`General_Category` != `Private_Use`), in the supported version"] # [doc = " of Unicode."] pub use crate :: tables :: is_public_assigned ; }
};
}
