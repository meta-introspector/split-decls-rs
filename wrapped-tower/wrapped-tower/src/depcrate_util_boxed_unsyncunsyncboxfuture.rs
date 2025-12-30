// Generated macro for UnsyncBoxFuture (type)
macro_rules! Depcrate_util_boxed_unsyncUnsyncBoxFuture {
() => {
// Module: crate::util::boxed::unsync
// Provides: {"UnsyncBoxFuture"}
// Dependencies: {}
# [doc = " A boxed [`Future`] trait object."] # [doc = ""] # [doc = " This type alias represents a boxed future that is *not* [`Send`] and must"] # [doc = " remain on the current thread."] type UnsyncBoxFuture < T , E > = Pin < Box < dyn Future < Output = Result < T , E > > > > ;
};
}
