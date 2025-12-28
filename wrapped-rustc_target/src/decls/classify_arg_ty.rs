macro_rules! deps {
    () => {
        ArgAbi!();
        Uniform!();
    };
}

macro_rules! classify_arg_ty {
    () => {
        deps!();
        fn classify_arg_ty < 'a , Ty , C > (arg : & mut ArgAbi < '_ , Ty > , arg_gprs_left : & mut u64 , max_size : u64) where Ty : TyAbiInterface < 'a , C > + Copy , { assert ! (* arg_gprs_left <= NUM_ARG_GPRS , "Arg GPR tracking underflow") ; if arg . layout . is_zst () { return ; } let size = arg . layout . size . bits () ; let needed_align = arg . layout . align . abi . bits () ; let mut must_use_stack = false ; let mut needed_arg_gprs = size . div_ceil (32) ; if needed_align == 64 { needed_arg_gprs += * arg_gprs_left % 2 ; } if needed_arg_gprs > * arg_gprs_left || needed_align > 128 || (* arg_gprs_left < (max_size / 32) && needed_align == 128) { must_use_stack = true ; needed_arg_gprs = * arg_gprs_left ; } * arg_gprs_left -= needed_arg_gprs ; if must_use_stack { arg . pass_by_stack_offset (None) ; } else if is_xtensa_aggregate (arg) { if size <= 32 { arg . cast_to (Reg :: i32 ()) ; } else { let reg = if needed_align == 2 * 32 { Reg :: i64 () } else { Reg :: i32 () } ; let total = Size :: from_bits (((size + 32 - 1) / 32) * 32) ; arg . cast_to (Uniform :: new (reg , total)) ; } } else { if size < 32 { arg . extend_integer_width_to (32) ; } } }
    };
}

classify_arg_ty!()