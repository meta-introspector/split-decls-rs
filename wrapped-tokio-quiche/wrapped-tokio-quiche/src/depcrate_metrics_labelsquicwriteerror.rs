// Generated macro for QuicWriteError (enum)
macro_rules! Depcrate_metrics_labelsQuicWriteError {
() => {
// Module: crate::metrics::labels
// Provides: {"QuicWriteError"}
// Dependencies: {}
# [doc = " Type of UDP [`send(2)`](https://man7.org/linux/man-pages/man2/send.2.html) error observed."] # [derive (Clone , Eq , Hash , PartialEq , Serialize)] # [serde (rename_all = "lowercase")] pub enum QuicWriteError { Err , Partial , WouldBlock , }
};
}
