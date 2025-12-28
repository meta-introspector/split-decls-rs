macro_rules! CompoundRepr {
    () => {
        # [doc = " A compound representation: `repr(C)`, `repr(Rust)`, or `repr(Int)`."] # [cfg_attr (test , derive (Copy , Clone , Debug , Eq , PartialEq))] pub (crate) enum CompoundRepr < Prim > { C , Rust , Primitive (Prim) , }
    };
}

CompoundRepr!();