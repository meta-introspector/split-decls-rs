macro_rules! uses_vector_registers {
    () => {
        fn uses_vector_registers (mode : & PassMode , repr : & BackendRepr) -> bool { match mode { PassMode :: Ignore | PassMode :: Indirect { .. } => false , PassMode :: Cast { pad_i32 : _ , cast } => { cast . prefix . iter () . any (| r | r . is_some_and (| x | x . kind == RegKind :: Vector)) || cast . rest . unit . kind == RegKind :: Vector } PassMode :: Direct (..) | PassMode :: Pair (..) => matches ! (repr , BackendRepr :: SimdVector { .. }) , } }
    };
}

uses_vector_registers!();