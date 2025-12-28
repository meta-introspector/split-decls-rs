macro_rules! deps {
    () => {
        Memory!();
        Sdata!();
    };
}

macro_rules! parse_structure {
    () => {
        deps!();
        fn parse_structure < 'a , Ty , C > (cx : & C , layout : TyAndLayout < 'a , Ty > , mut data : Sdata , mut offset : Size ,) -> Sdata where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if let FieldsShape :: Union (_) = layout . fields { return data ; } match layout . backend_repr { BackendRepr :: Scalar (scalar) => { data = arg_scalar (cx , & scalar , offset , data) ; } BackendRepr :: Memory { .. } => { for i in 0 .. layout . fields . count () { if offset < layout . fields . offset (i) { offset = layout . fields . offset (i) ; } data = parse_structure (cx , layout . field (cx , i) , data . clone () , offset) ; } } _ => { if let BackendRepr :: ScalarPair (scalar1 , scalar2) = & layout . backend_repr { data = arg_scalar_pair (cx , scalar1 , scalar2 , offset , data) ; } } } data }
    };
}

parse_structure!()