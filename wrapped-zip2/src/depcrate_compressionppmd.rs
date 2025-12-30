// Generated macro for Ppmd (enum)
macro_rules! Depcrate_compressionPpmd {
() => {
// Module: crate::compression
// Provides: {"Ppmd"}
// Dependencies: {}
# [cfg (feature = "ppmd")] pub (crate) enum Ppmd < R : io :: BufRead > { Uninitialized (Option < R >) , Initialized (Box < ppmd_rust :: Ppmd8Decoder < R > >) , }
};
}
