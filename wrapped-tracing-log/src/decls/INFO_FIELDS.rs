macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! INFO_FIELDS {
    () => {
        deps!();
        static INFO_FIELDS : Lazy < Fields > = Lazy :: new (| | Fields :: new (& INFO_CS)) ;
    };
}

INFO_FIELDS!()