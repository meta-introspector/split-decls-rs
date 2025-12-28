macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! TRACE_FIELDS {
    () => {
        deps!();
        static TRACE_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& TRACE_CS)) ;
    };
}

TRACE_FIELDS!();