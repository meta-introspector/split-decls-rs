// Generated macro for impl_119 (impl)
macro_rules! Depcrate_dispatcherimpl_119 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_119"}
// Dependencies: {}
impl WeakDispatch { # [doc = " Attempts to upgrade this `WeakDispatch` to a [`Dispatch`]."] # [doc = ""] # [doc = " Returns `None` if the referenced `Dispatch` has already been dropped."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use tracing_core::subscriber::NoSubscriber;"] # [doc = " # use tracing_core::dispatcher::Dispatch;"] # [doc = " let strong = Dispatch::new(NoSubscriber::default());"] # [doc = " let weak = strong.downgrade();"] # [doc = ""] # [doc = " // The strong here keeps it alive, so we can still access the object."] # [doc = " assert!(weak.upgrade().is_some());"] # [doc = ""] # [doc = " drop(strong); // But not any more."] # [doc = " assert!(weak.upgrade().is_none());"] # [doc = " ```"] pub fn upgrade (& self) -> Option < Dispatch > { self . subscriber . upgrade () . map (| subscriber | Dispatch { subscriber }) } }
};
}
