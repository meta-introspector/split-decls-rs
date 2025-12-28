macro_rules! deps {
    () => {
        AlignRepr!();
        CompoundRepr!();
    };
}

macro_rules! Repr {
    () => {
        deps!();
        # [doc = " The computed representation of a type."] # [doc = ""] # [doc = " This is the result of processing all `#[repr(...)]` attributes on a type, if"] # [doc = " any. A `Repr` is only capable of representing legal combinations of"] # [doc = " `#[repr(...)]` attributes."] # [cfg_attr (test , derive (Copy , Clone , Debug))] pub (crate) enum Repr < Prim , Packed > { # [doc = " `#[repr(transparent)]`"] Transparent (Span) , # [doc = " A compound representation: `repr(C)`, `repr(Rust)`, or `repr(Int)`"] # [doc = " optionally combined with `repr(packed(...))` or `repr(align(...))`"] Compound (Spanned < CompoundRepr < Prim > > , Option < Spanned < AlignRepr < Packed > > >) , }
    };
}

Repr!();