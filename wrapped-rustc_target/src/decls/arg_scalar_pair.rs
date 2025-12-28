macro_rules! deps {
    () => {
        Sdata!();
    };
}

macro_rules! arg_scalar_pair {
    () => {
        deps!();
        fn arg_scalar_pair < C > (cx : & C , scalar1 : & Scalar , scalar2 : & Scalar , mut offset : Size , mut data : Sdata ,) -> Sdata where C : HasDataLayout , { data = arg_scalar (cx , scalar1 , offset , data) ; match (scalar1 . primitive () , scalar2 . primitive ()) { (Primitive :: Float (Float :: F32) , _) => offset += Reg :: f32 () . size , (_ , Primitive :: Float (Float :: F64)) => offset += Reg :: f64 () . size , (Primitive :: Int (i , _signed) , _) => offset += i . size () , (Primitive :: Pointer (_) , _) => offset += Reg :: i64 () . size , _ => { } } if ! offset . bytes () . is_multiple_of (4) && matches ! (scalar2 . primitive () , Primitive :: Float (Float :: F32 | Float :: F64)) { offset += Size :: from_bytes (4 - (offset . bytes () % 4)) ; } data = arg_scalar (cx , scalar2 , offset , data) ; data }
    };
}

arg_scalar_pair!()