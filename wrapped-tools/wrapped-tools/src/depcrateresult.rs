// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A result type to allow using the try operator `?` in unit tests."] # [doc = ""] # [doc = " Use it like so:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gix_testtools::Result;"] # [doc = ""] # [doc = " #[test]"] # [doc = " fn this() -> Result {"] # [doc = "     let x: usize = \"42\".parse()?;"] # [doc = "     Ok(())"] # [doc = ""] # [doc = " }"] # [doc = " ```"] pub type Result < T = () > = std :: result :: Result < T , Box < dyn std :: error :: Error + Send + Sync > > ;
};
}
