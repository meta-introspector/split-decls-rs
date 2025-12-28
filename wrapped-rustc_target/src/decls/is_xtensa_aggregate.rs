macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! is_xtensa_aggregate {
    () => {
        deps!();
        fn is_xtensa_aggregate < 'a , Ty > (arg : & ArgAbi < 'a , Ty >) -> bool { match arg . layout . backend_repr { BackendRepr :: SimdVector { .. } => true , _ => arg . layout . is_aggregate () , } }
    };
}

is_xtensa_aggregate!()