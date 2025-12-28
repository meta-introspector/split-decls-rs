macro_rules! deps {
    () => {
        CastTarget!();
        ArgAbi!();
        PassMode!();
        Uniform!();
    };
}

macro_rules! classify_arg_kernel {
    () => {
        deps!();
        fn classify_arg_kernel < 'a , Ty , C > (_cx : & C , arg : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { match arg . mode { super :: PassMode :: Ignore | super :: PassMode :: Direct (_) => return , super :: PassMode :: Pair (_ , _) => { } super :: PassMode :: Cast { .. } => unreachable ! () , super :: PassMode :: Indirect { .. } => { } } let align_bytes = arg . layout . align . abi . bytes () ; let unit = match align_bytes { 1 => Reg :: i8 () , 2 => Reg :: i16 () , 4 => Reg :: i32 () , 8 => Reg :: i64 () , 16 => Reg :: i128 () , _ => unreachable ! ("Align is given as power of 2 no larger than 16 bytes") , } ; if arg . layout . size . bytes () / align_bytes == 1 { arg . cast_to (CastTarget :: prefixed ([Some (unit) , None , None , None , None , None , None , None] , Uniform :: new (unit , Size :: ZERO) ,)) ; } else { arg . cast_to (Uniform :: new (unit , arg . layout . size)) ; } }
    };
}

classify_arg_kernel!()