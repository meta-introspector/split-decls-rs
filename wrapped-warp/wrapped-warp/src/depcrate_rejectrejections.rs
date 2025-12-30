// Generated macro for Rejections (enum)
macro_rules! Depcrate_rejectRejections {
() => {
// Module: crate::reject
// Provides: {"Rejections"}
// Dependencies: {}
enum Rejections { Known (Known) , Custom (Box < dyn Cause >) , Combined (Box < Rejections > , Box < Rejections >) , }
};
}
