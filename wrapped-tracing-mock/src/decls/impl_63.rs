macro_rules! deps {
    () => {
        ExpectedId!();
        SetActualSpanIdError!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl ExpectedId { const UNSET : u64 = 0 ; pub (crate) fn new_unset () -> Self { Self { inner : Arc :: new (AtomicU64 :: from (Self :: UNSET)) , } } pub (crate) fn set (& self , span_id : u64) -> Result < () , SetActualSpanIdError > { self . inner . compare_exchange (Self :: UNSET , span_id , Ordering :: Relaxed , Ordering :: Relaxed) . map_err (| current | SetActualSpanIdError { previous_span_id : current , new_span_id : span_id , }) ? ; Ok (()) } pub (crate) fn check (& self , actual : & tracing_core :: span :: Id , ctx : fmt :: Arguments < '_ > , subscriber_name : & str ,) { let expected_id = self . inner . load (Ordering :: Relaxed) ; let actual_id = actual . into_u64 () ; assert ! (expected_id != Self :: UNSET , "{}" , format ! ("\n[{subscriber_name}] expected {ctx} with an expected Id set,\n\
                [{subscriber_name}] but it hasn't been, perhaps this `ExpectedId` \
                wasn't used in a call to `new_span()`?")) ; assert_eq ! (expected_id , actual_id , "{}" , format_args ! ("\n[{subscriber_name}] expected {ctx} with Id `{expected_id}`,\n\
                [{subscriber_name}] but got one with Id `{actual_id}` instead" ,)) ; } }
    };
}

impl_63!()