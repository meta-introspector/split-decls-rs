// Generated macro for impl_1499 (impl)
macro_rules! Depcrate_utilimpl_1499 {
() => {
// Module: crate::util
// Provides: {"impl_1499"}
// Dependencies: {}
impl MetadataCtimeExt for std :: fs :: Metadata { # [cfg (unix)] fn ctime_or_creation (& self) -> std :: io :: Result < Timestamp > { use std :: os :: unix :: prelude :: MetadataExt ; Ok (Timestamp { seconds : self . ctime () , nanoseconds : self . ctime_nsec () . try_into () . unwrap_or (0) , }) } # [cfg (windows)] fn ctime_or_creation (& self) -> std :: io :: Result < Timestamp > { self . created () . map (Into :: into) } }
};
}
