// Generated macro for impl_41 (impl)
macro_rules! Depcrate_schemaimpl_41 {
() => {
// Module: crate::schema
// Provides: {"impl_41"}
// Dependencies: {}
impl std :: str :: FromStr for CommandStatus { type Err = crate :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "success" => Ok (Self :: Success) , "failed" => Ok (Self :: Failed) , "interrupted" => Ok (Self :: Interrupted) , "skipped" => Ok (Self :: Skipped) , _ => s . parse :: < i32 > () . map (Self :: Code) . map_err (| _ | crate :: Error :: new (format ! ("Expected an exit code, got {s}"))) , } } }
};
}
