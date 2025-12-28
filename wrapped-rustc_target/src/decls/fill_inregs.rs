macro_rules! deps {
    () => {
        Flavor!();
        FnAbi!();
        PassMode!();
        X86Options!();
    };
}

macro_rules! fill_inregs {
    () => {
        deps!();
        pub (crate) fn fill_inregs < 'a , Ty , C > (cx : & C , fn_abi : & mut FnAbi < 'a , Ty > , opts : X86Options , rust_abi : bool ,) where Ty : TyAbiInterface < 'a , C > + Copy , { if opts . flavor != Flavor :: FastcallOrVectorcall && opts . regparm . is_none_or (| x | x == 0) { return ; } let mut free_regs = opts . regparm . unwrap_or (2) . into () ; let has_casts = fn_abi . args . iter () . any (| arg | matches ! (arg . mode , PassMode :: Cast { .. })) ; if has_casts && rust_abi { return ; } for arg in fn_abi . args . iter_mut () { let attrs = match arg . mode { PassMode :: Ignore | PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } => { continue ; } PassMode :: Direct (ref mut attrs) => attrs , PassMode :: Pair (..) | PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } | PassMode :: Cast { .. } => { unreachable ! ("x86 shouldn't be passing arguments by {:?}" , arg . mode) } } ; let unit = arg . layout . homogeneous_aggregate (cx) . unwrap () . unit () . unwrap () ; assert_eq ! (unit . size , arg . layout . size) ; if matches ! (unit . kind , RegKind :: Float | RegKind :: Vector) { continue ; } let size_in_regs = arg . layout . size . bits () . div_ceil (32) ; if size_in_regs == 0 { continue ; } if size_in_regs > free_regs { break ; } free_regs -= size_in_regs ; if arg . layout . size . bits () <= 32 && unit . kind == RegKind :: Integer { attrs . set (ArgAttribute :: InReg) ; } if free_regs == 0 { break ; } } }
    };
}

fill_inregs!();