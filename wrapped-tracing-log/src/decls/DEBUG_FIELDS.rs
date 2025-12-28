macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! DEBUG_FIELDS {
    () => {
        deps!();
        static DEBUG_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& DEBUG_CS)) ;
    };
}

DEBUG_FIELDS!()