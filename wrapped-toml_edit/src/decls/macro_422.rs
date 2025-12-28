macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_422 {
    () => {
        deps!();
        empty_visit_mut ! (visit_string_mut , Formatted < String >) ;
    };
}

macro_422!();