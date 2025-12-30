// Generated macro for write_immediate (function)
macro_rules! Depcrate_layerwrite_immediate {
() => {
// Module: crate::layer
// Provides: {"write_immediate"}
// Dependencies: {}
fn write_immediate < S > (event : & tree :: Event , current : Option < & SpanRef < S > >) -> io :: Result < () > where S : for < 'a > LookupSpan < 'a > , { # [cfg (feature = "smallvec")] let mut writer = smallvec :: SmallVec :: < [u8 ; 256] > :: new () ; # [cfg (not (feature = "smallvec"))] let mut writer = Vec :: with_capacity (256) ; # [cfg (feature = "uuid")] if let Some (span) = current { let uuid = span . extensions () . get :: < OpenedSpan > () . expect (fail :: OPENED_SPAN_NOT_IN_EXTENSIONS) . span . uuid () ; write ! (writer , "{uuid} ") ? ; } # [cfg (feature = "chrono")] write ! (writer , "{} " , event . timestamp () . to_rfc3339 ()) ? ; write ! (writer , "{:<8} " , event . level ()) ? ; let tag = event . tag () . unwrap_or_else (| | Tag :: from (event . level ())) ; write ! (writer , "{icon} IMMEDIATE {icon} " , icon = tag . icon ()) ? ; if let Some (span) = current { for ancestor in span . scope () . from_root () { write ! (writer , "{} > " , ancestor . name ()) ? ; } } if let Some (message) = event . message () { write ! (writer , "{message}") ? ; } for field in event . fields () { write ! (writer , " | {}: {}" , field . key () , field . value ()) ? ; } writeln ! (writer) ? ; io :: stderr () . write_all (& writer) }
};
}
