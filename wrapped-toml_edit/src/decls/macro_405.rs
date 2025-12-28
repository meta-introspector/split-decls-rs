macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_405 {
    () => {
        deps!();
        empty_visit ! (visit_string , Formatted < String >) ;
    };
}

macro_405!()