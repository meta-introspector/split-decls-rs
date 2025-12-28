macro_rules! deps {
    () => {
        RegPassKind!();
        FloatConv!();
    };
}

macro_rules! should_use_fp_conv {
    () => {
        deps!();
        fn should_use_fp_conv < 'a , Ty , C > (cx : & C , arg : & TyAndLayout < 'a , Ty > , xlen : u64 , flen : u64 ,) -> Option < FloatConv > where Ty : TyAbiInterface < 'a , C > + Copy , { let mut field1_kind = RegPassKind :: Unknown ; let mut field2_kind = RegPassKind :: Unknown ; if should_use_fp_conv_helper (cx , arg , xlen , flen , & mut field1_kind , & mut field2_kind , Size :: ZERO ,) . is_err () { return None ; } match (field1_kind , field2_kind) { (RegPassKind :: Integer { offset_from_start , .. } | RegPassKind :: Float { offset_from_start , .. } , _ ,) if offset_from_start != Size :: ZERO => { panic ! ("type {:?} has a first field with non-zero offset {offset_from_start:?}" , arg . ty) } (RegPassKind :: Integer { ty : first_ty , .. } , RegPassKind :: Float { offset_from_start , ty : second_ty } ,) => Some (FloatConv :: MixedPair { first_ty , second_ty_offset_from_start : offset_from_start , second_ty , }) , (RegPassKind :: Float { ty : first_ty , .. } , RegPassKind :: Integer { offset_from_start , ty : second_ty } ,) => Some (FloatConv :: MixedPair { first_ty , second_ty_offset_from_start : offset_from_start , second_ty , }) , (RegPassKind :: Float { ty : first_ty , .. } , RegPassKind :: Float { offset_from_start , ty : second_ty } ,) => Some (FloatConv :: FloatPair { first_ty , second_ty_offset_from_start : offset_from_start , second_ty , }) , (RegPassKind :: Float { ty , .. } , RegPassKind :: Unknown) => Some (FloatConv :: Float (ty)) , _ => None , } }
    };
}

should_use_fp_conv!()