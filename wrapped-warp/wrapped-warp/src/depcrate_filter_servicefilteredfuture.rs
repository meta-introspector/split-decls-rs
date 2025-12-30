// Generated macro for FilteredFuture (struct)
macro_rules! Depcrate_filter_serviceFilteredFuture {
() => {
// Module: crate::filter::service
// Provides: {"FilteredFuture"}
// Dependencies: {}
# [pin_project] # [derive (Debug)] pub struct FilteredFuture < F > { # [pin] future : F , route : :: std :: cell :: RefCell < Route > , }
};
}
