macro_rules! deps {
    () => {
        MaybeInitializedPlaces!();
        MoveData!();
    };
}

macro_rules! MaybeUninitializedPlaces {
    () => {
        deps!();
        # [doc = " `MaybeUninitializedPlaces` tracks all places that might be"] # [doc = " uninitialized upon reaching a particular point in the control flow"] # [doc = " for a function."] # [doc = ""] # [doc = " For example, in code like the following, we have corresponding"] # [doc = " dataflow information shown in the right-hand comments."] # [doc = ""] # [doc = " ```rust"] # [doc = " struct S;"] # [doc = " #[rustfmt::skip]"] # [doc = " fn foo(pred: bool) {                        // maybe-uninit:"] # [doc = "                                             // {a, b, c, d}"] # [doc = "     let a = S; let mut b = S; let c; let d; // {      c, d}"] # [doc = ""] # [doc = "     if pred {"] # [doc = "         drop(a);                            // {a,    c, d}"] # [doc = "         b = S;                              // {a,    c, d}"] # [doc = ""] # [doc = "     } else {"] # [doc = "         drop(b);                            // {   b, c, d}"] # [doc = "         d = S;                              // {   b, c   }"] # [doc = ""] # [doc = "     }                                       // {a, b, c, d}"] # [doc = ""] # [doc = "     c = S;                                  // {a, b,    d}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " To determine whether a place is *definitely* uninitialized at a"] # [doc = " particular control-flow point, one can take the set-complement"] # [doc = " of the data from `MaybeInitializedPlaces` at the corresponding"] # [doc = " control-flow point."] # [doc = ""] # [doc = " Similarly, at a given `drop` statement, the set-intersection"] # [doc = " between this data and `MaybeInitializedPlaces` yields the set of"] # [doc = " places that would require a dynamic drop-flag at that statement."] pub struct MaybeUninitializedPlaces < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx > , mark_inactive_variants_as_uninit : bool , include_inactive_in_otherwise : bool , skip_unreachable_unwind : DenseBitSet < mir :: BasicBlock > , }
    };
}

MaybeUninitializedPlaces!();