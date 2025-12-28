macro_rules! deps {
    () => {
        ArgAbi!();
        Uniform!();
        ABI!();
    };
}

macro_rules! classify {
    () => {
        deps!();
        fn classify < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , abi : ABI , is_ret : bool) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if arg . is_ignore () || ! arg . layout . is_sized () { return ; } if ! arg . layout . is_aggregate () { arg . extend_integer_width_to (64) ; return ; } if ! is_ret && abi == AIX { arg . pass_by_stack_offset (Some (Align :: from_bytes (8) . unwrap ())) ; return ; } if is_ret && (abi == ELFv1 || abi == AIX) { arg . make_indirect () ; return ; } if let Some (uniform) = is_homogeneous_aggregate (cx , arg , abi) { arg . cast_to (uniform) ; return ; } let size = arg . layout . size ; if is_ret && size . bits () > 128 { arg . make_indirect () ; } else if size . bits () <= 64 { arg . cast_to (Reg { kind : RegKind :: Integer , size }) } else { let reg = if arg . layout . align . abi . bytes () > 8 { Reg :: i128 () } else { Reg :: i64 () } ; arg . cast_to (Uniform :: consecutive (reg , size . align_to (Align :: from_bytes (reg . size . bytes ()) . unwrap ()) ,)) } ; }
    };
}

classify!();