macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_421 {
    () => {
        deps!();
        empty_visit_mut ! (visit_integer_mut , Formatted < i64 >) ;
    };
}

macro_421!();