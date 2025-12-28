macro_rules! deps {
    () => {
        ArgAbi!();
    };
}

macro_rules! is_riscv_aggregate {
    () => {
        deps!();
        fn is_riscv_aggregate < Ty > (arg : & ArgAbi < '_ , Ty >) -> bool { match arg . layout . backend_repr { BackendRepr :: SimdVector { .. } => true , _ => arg . layout . is_aggregate () , } }
    };
}

is_riscv_aggregate!()