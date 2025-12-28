macro_rules! InvariantsEq {
    () => {
        # [doc = " Denotes that two types have the same invariants."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is sound for safe code to operate on a `&T` and a `&Self` pointing to the"] # [doc = " same referent at the same time - no such safe code can cause undefined"] # [doc = " behavior."] pub unsafe trait InvariantsEq < T : ? Sized > { }
    };
}

InvariantsEq!()