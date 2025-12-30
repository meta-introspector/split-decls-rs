// Generated macro for transform_mut_yokeable (function)
macro_rules! Depcrate_utilstransform_mut_yokeable {
() => {
// Module: crate::utils
// Provides: {"transform_mut_yokeable"}
// Dependencies: {}
# [doc = " This method casts `yokeable` between `&'a mut Y<'static>` and `&'a mut Y<'a>`,"] # [doc = " and passes it to `f`."] # [doc = ""] # [doc = " See [`Yokeable::transform_mut`] for why this is safe, noting that no `'static` return type"] # [doc = " can leak data from the cart or Yokeable."] # [inline] pub (crate) fn transform_mut_yokeable < 'a , Y , F , R > (yokeable : & 'a mut Y , f : F) -> R where Y : Yokeable < 'a > , F : 'static + for < 'b > FnOnce (& 'b mut Y :: Output) -> R , R : 'static , { unsafe { f (mem :: transmute :: < & 'a mut Y , & 'a mut Y :: Output > (yokeable)) } }
};
}
