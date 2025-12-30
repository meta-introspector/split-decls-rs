// Generated macro for ReusableBoxFuture (struct)
macro_rules! Depcrate_sync_reusable_boxReusableBoxFuture {
() => {
// Module: crate::sync::reusable_box
// Provides: {"ReusableBoxFuture"}
// Dependencies: {}
# [doc = " A reusable `Pin<Box<dyn Future<Output = T> + Send + 'a>>`."] # [doc = ""] # [doc = " This type lets you replace the future stored in the box without"] # [doc = " reallocating when the size and alignment permits this."] pub struct ReusableBoxFuture < 'a , T > { boxed : Pin < Box < dyn Future < Output = T > + Send + 'a > > , }
};
}
