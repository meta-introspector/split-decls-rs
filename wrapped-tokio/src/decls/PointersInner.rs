macro_rules! deps {
    () => {
        Pointers!();
    };
}

macro_rules! PointersInner {
    () => {
        deps!();
        # [doc = " We do not want the compiler to put the `noalias` attribute on mutable"] # [doc = " references to this type, so the type has been made `!Unpin` with a"] # [doc = " `PhantomPinned` field."] # [doc = ""] # [doc = " Additionally, we never access the `prev` or `next` fields directly, as any"] # [doc = " such access would implicitly involve the creation of a reference to the"] # [doc = " field, which we want to avoid since the fields are not `!Unpin`, and would"] # [doc = " hence be given the `noalias` attribute if we were to do such an access. As"] # [doc = " an alternative to accessing the fields directly, the `Pointers` type"] # [doc = " provides getters and setters for the two fields, and those are implemented"] # [doc = " using `ptr`-specific methods which avoids the creation of intermediate"] # [doc = " references."] # [doc = ""] # [doc = " See this link for more information:"] # [doc = " <https://github.com/rust-lang/rust/pull/82834>"] struct PointersInner < T > { # [doc = " The previous node in the list. null if there is no previous node."] prev : Option < NonNull < T > > , # [doc = " The next node in the list. null if there is no previous node."] next : Option < NonNull < T > > , # [doc = " This type is !Unpin due to the heuristic from:"] # [doc = " <https://github.com/rust-lang/rust/pull/82834>"] _pin : PhantomPinned , }
    };
}

PointersInner!()