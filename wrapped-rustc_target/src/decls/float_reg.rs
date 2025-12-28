macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! float_reg {
    () => {
        deps!();
        fn float_reg < 'a , Ty , C > (cx : & C , ret : & ArgAbi < 'a , Ty > , i : usize) -> Option < Reg > where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { match ret . layout . field (cx , i) . backend_repr { BackendRepr :: Scalar (scalar) => match scalar . primitive () { Primitive :: Float (Float :: F32) => Some (Reg :: f32 ()) , Primitive :: Float (Float :: F64) => Some (Reg :: f64 ()) , _ => None , } , _ => None , } }
    };
}

float_reg!();