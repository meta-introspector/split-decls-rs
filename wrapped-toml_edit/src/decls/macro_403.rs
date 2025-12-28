macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_403 {
    () => {
        deps!();
        empty_visit ! (visit_float , Formatted < f64 >) ;
    };
}

macro_403!();