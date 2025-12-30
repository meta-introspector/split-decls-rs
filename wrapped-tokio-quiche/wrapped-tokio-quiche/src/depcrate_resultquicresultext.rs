// Generated macro for QuicResultExt (trait)
macro_rules! Depcrate_resultQuicResultExt {
() => {
// Module: crate::result
// Provides: {"QuicResultExt"}
// Dependencies: {}
# [doc = " Extension trait to add methods to [Result]."] pub trait QuicResultExt < T , E > { # [doc = " Turns the [Result] into an [`io::Result`] with"] # [doc = " [`ErrorKind::Other`](io::ErrorKind::Other)."] fn into_io (self) -> io :: Result < T > where E : Into < BoxError > ; }
};
}
