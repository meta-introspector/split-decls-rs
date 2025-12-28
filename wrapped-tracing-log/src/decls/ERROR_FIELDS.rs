macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! ERROR_FIELDS {
    () => {
        deps!();
        static ERROR_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& ERROR_CS)) ;
    };
}

ERROR_FIELDS!();