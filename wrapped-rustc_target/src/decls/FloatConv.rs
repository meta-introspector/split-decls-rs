macro_rules! FloatConv {
    () => {
        # [derive (Copy , Clone)] enum FloatConv { FloatPair { first_ty : Reg , second_ty_offset_from_start : Size , second_ty : Reg } , Float (Reg) , MixedPair { first_ty : Reg , second_ty_offset_from_start : Size , second_ty : Reg } , }
    };
}

FloatConv!()