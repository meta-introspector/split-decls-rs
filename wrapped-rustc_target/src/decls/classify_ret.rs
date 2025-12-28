macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! classify_ret {
    () => {
        deps!();
        fn classify_ret < 'a , Ty , C > (cx : & C , ret : & mut ArgAbi < 'a , Ty >) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { ret . extend_integer_width_to (32) ; if ret . layout . is_aggregate () && ! unwrap_trivial_aggregate (cx , ret) { ret . make_indirect () ; } if let BackendRepr :: Scalar (scalar) = ret . layout . backend_repr { match scalar . primitive () { Primitive :: Int (Integer :: I128 , _) | Primitive :: Float (Float :: F128) => { ret . make_indirect () ; } _ => { } } } }
    };
}

classify_ret!()