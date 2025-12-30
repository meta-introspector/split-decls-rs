// Generated macro for impl_284 (impl)
macro_rules! Depcrate_cmdlineimpl_284 {
() => {
// Module: crate::cmdline
// Provides: {"impl_284"}
// Dependencies: {}
impl FromStr for StatsFormat { type Err = anyhow :: Error ; fn from_str (s : & str) -> anyhow :: Result < Self > { match s { "text" => Ok (Self :: Text) , "json" => Ok (Self :: Json) , _ => bail ! ("Unrecognized stats format: {:?}" , s) , } } }
};
}
