macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_418 {
    () => {
        deps!();
        empty_visit_mut ! (visit_boolean_mut , Formatted < bool >) ;
    };
}

macro_418!()