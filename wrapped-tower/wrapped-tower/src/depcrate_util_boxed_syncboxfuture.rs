// Generated macro for BoxFuture (type)
macro_rules! Depcrate_util_boxed_syncBoxFuture {
() => {
// Module: crate::util::boxed::sync
// Provides: {"BoxFuture"}
// Dependencies: {}
# [doc = " A boxed `Future + Send` trait object."] # [doc = ""] # [doc = " This type alias represents a boxed future that is [`Send`] and can be moved"] # [doc = " across threads."] type BoxFuture < T , E > = Pin < Box < dyn Future < Output = Result < T , E > > + Send > > ;
};
}
