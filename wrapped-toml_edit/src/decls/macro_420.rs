macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_420 {
    () => {
        deps!();
        empty_visit_mut ! (visit_float_mut , Formatted < f64 >) ;
    };
}

macro_420!()