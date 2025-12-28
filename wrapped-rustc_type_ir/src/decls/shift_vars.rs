macro_rules! deps {
    () => {
        Shifter!();
        Interner!();
        TypeFoldable!();
    };
}

macro_rules! shift_vars {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (cx) , ret)] pub fn shift_vars < I : Interner , T > (cx : I , value : T , amount : u32) -> T where T : TypeFoldable < I > , { if amount == 0 || ! value . has_escaping_bound_vars () { value } else { value . fold_with (& mut Shifter :: new (cx , amount)) } }
    };
}

shift_vars!();