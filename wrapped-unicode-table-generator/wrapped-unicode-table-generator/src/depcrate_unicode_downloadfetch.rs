// Generated macro for fetch (function)
macro_rules! Depcrate_unicode_downloadfetch {
() => {
// Module: crate::unicode_download
// Provides: {"fetch"}
// Dependencies: {}
# [track_caller] fn fetch (url : & str) -> Output { let output = Command :: new ("curl") . arg (URL_PREFIX . to_owned () + url) . output () . unwrap () ; if ! output . status . success () { panic ! ("Failed to run curl to fetch {url}: stderr: {}" , String :: from_utf8_lossy (& output . stderr)) ; } output }
};
}
