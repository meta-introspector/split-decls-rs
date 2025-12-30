// Generated macro for impl_91 (impl)
macro_rules! Depcrate_sinkimpl_91 {
() => {
// Module: crate::sink
// Provides: {"impl_91"}
// Dependencies: {}
# [doc = " A `Box<dyn std::error::Error>` can be used as an error for `Sink`"] # [doc = " implementations out of the box."] impl SinkError for Box < dyn std :: error :: Error > { fn error_message < T : std :: fmt :: Display > (message : T ,) -> Box < dyn std :: error :: Error > { Box :: < dyn std :: error :: Error > :: from (message . to_string ()) } }
};
}
