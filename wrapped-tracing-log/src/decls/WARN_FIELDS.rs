macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! WARN_FIELDS {
    () => {
        deps!();
        static WARN_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& WARN_CS)) ;
    };
}

WARN_FIELDS!()