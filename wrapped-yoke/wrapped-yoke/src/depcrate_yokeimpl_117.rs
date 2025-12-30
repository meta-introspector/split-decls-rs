// Generated macro for impl_117 (impl)
macro_rules! Depcrate_yokeimpl_117 {
() => {
// Module: crate::yoke
// Provides: {"impl_117"}
// Dependencies: {}
# [doc = " Clone requires that the cart type `C` derefs to the same address after it is cloned. This works for"] # [doc = " Rc, Arc, and &'a T."] # [doc = ""] # [doc = " For other cart types, clone `.backing_cart()` and re-use `.attach_to_cart()`; however, doing"] # [doc = " so may lose mutations performed via `.with_mut()`."] # [doc = ""] # [doc = " Cloning a `Yoke` is often a cheap operation requiring no heap allocations, in much the same"] # [doc = " way that cloning an `Rc` is a cheap operation. However, if the `yokeable` contains owned data"] # [doc = " (e.g., from `.with_mut()`), that data will need to be cloned."] impl < Y : for < 'a > Yokeable < 'a > , C : CloneableCart > Clone for Yoke < Y , C > where for < 'a > < Y as Yokeable < 'a > > :: Output : Clone , { fn clone (& self) -> Self { let this = self . get () . clone () ; Yoke { yokeable : KindaSortaDangling :: new (unsafe { Y :: make (this) } ,) , cart : self . cart . clone () , } } }
};
}
