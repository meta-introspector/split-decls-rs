macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_404 {
    () => {
        deps!();
        empty_visit ! (visit_integer , Formatted < i64 >) ;
    };
}

macro_404!()