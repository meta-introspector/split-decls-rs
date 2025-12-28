macro_rules! deps {
    () => {
        CastTarget!();
        Uniform!();
        ArgAbi!();
    };
}

macro_rules! classify_aggregate {
    () => {
        deps!();
        # [doc = " the pass mode used for aggregates in arg and ret position"] fn classify_aggregate < Ty > (arg : & mut ArgAbi < '_ , Ty >) { let align_bytes = arg . layout . align . abi . bytes () ; let size = arg . layout . size ; let reg = match align_bytes { 1 => Reg :: i8 () , 2 => Reg :: i16 () , 4 => Reg :: i32 () , 8 => Reg :: i64 () , 16 => Reg :: i128 () , _ => unreachable ! ("Align is given as power of 2 no larger than 16 bytes") , } ; if align_bytes == size . bytes () { arg . cast_to (CastTarget :: prefixed ([Some (reg) , None , None , None , None , None , None , None] , Uniform :: new (Reg :: i8 () , Size :: ZERO) ,)) ; } else { arg . cast_to (Uniform :: new (reg , size)) ; } }
    };
}

classify_aggregate!();