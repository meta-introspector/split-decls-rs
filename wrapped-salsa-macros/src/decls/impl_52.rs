macro_rules! deps {
    () => {
        Options!();
        AllowedOptions!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < A : AllowedOptions > Default for Options < A > { fn default () -> Self { Self { returns : Default :: default () , specify : Default :: default () , non_update_return_type : Default :: default () , no_eq : Default :: default () , debug : Default :: default () , no_lifetime : Default :: default () , db_path : Default :: default () , cycle_fn : Default :: default () , cycle_initial : Default :: default () , cycle_result : Default :: default () , data : Default :: default () , constructor_name : Default :: default () , phantom : Default :: default () , lru : Default :: default () , singleton : Default :: default () , id : Default :: default () , revisions : Default :: default () , heap_size_fn : Default :: default () , self_ty : Default :: default () , persist : Default :: default () , } } }
    };
}

impl_52!();