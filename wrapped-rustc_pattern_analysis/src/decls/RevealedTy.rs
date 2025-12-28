macro_rules! RevealedTy {
    () => {
        # [doc = " A type which has gone through `cx.reveal_opaque_ty`, i.e. if it was opaque it was replaced by"] # [doc = " the hidden type if allowed in the current body. This ensures we consistently inspect the hidden"] # [doc = " types when we should."] # [doc = ""] # [doc = " Use `.inner()` or deref to get to the `Ty<'tcx>`."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct RevealedTy < 'tcx > (Ty < 'tcx >) ;
    };
}

RevealedTy!()