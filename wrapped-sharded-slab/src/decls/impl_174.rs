macro_rules! deps {
    () => {
        Registration!();
        Config!();
        Tid!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl Registration { fn new () -> Self { Self (Cell :: new (None)) } # [inline (always)] fn current < C : cfg :: Config > (& self) -> Tid < C > { if let Some (tid) = self . 0 . get () . map (Tid :: new) { return tid ; } self . register () } # [cold] fn register < C : cfg :: Config > (& self) -> Tid < C > { let id = REGISTRY . free . lock () . ok () . and_then (| mut free | { if free . len () > 1 { free . pop_front () } else { None } }) . unwrap_or_else (| | { let id = REGISTRY . next . fetch_add (1 , Ordering :: AcqRel) ; if id > Tid :: < C > :: BITS { panic_in_drop ! ("creating a new thread ID ({}) would exceed the \
                        maximum number of thread ID bits specified in {} \
                        ({})" , id , std :: any :: type_name ::< C > () , Tid ::< C >:: BITS ,) ; } id }) ; self . 0 . set (Some (id)) ; Tid :: new (id) } }
    };
}

impl_174!()