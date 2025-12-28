macro_rules! SizeEq {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " `T` and `Self` must have the same vtable kind (`Sized`, slice DST, `dyn`,"] # [doc = " etc) and have the same size. In particular:"] # [doc = " - If `T: Sized` and `Self: Sized`, then their sizes must be equal"] # [doc = " - If `T: ?Sized` and `Self: ?Sized`, then it must be the case that, given"] # [doc = "   any `t: PtrInner<'_, T>`, `<Self as SizeEq<T>>::cast_from_raw(t)` produces"] # [doc = "   a pointer which addresses the same number of bytes as `t`. *Note that it"] # [doc = "   is **not** guaranteed that an `as` cast preserves referent size: it may be"] # [doc = "   the case that `cast_from_raw` modifies the pointer's metadata in order to"] # [doc = "   preserve referent size, which an `as` cast does not do.*"] pub unsafe trait SizeEq < T : ? Sized > { fn cast_from_raw (t : PtrInner < '_ , T >) -> PtrInner < '_ , Self > ; }
    };
}

SizeEq!()