macro_rules! response_no_constraints {
    () => {
        fn response_no_constraints < I : Interner > (cx : I , input : CanonicalInput < I > , certainty : Certainty ,) -> QueryResult < I > { Ok (super :: response_no_constraints_raw (cx , input . canonical . max_universe , input . canonical . variables , certainty ,)) }
    };
}

response_no_constraints!()