macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_402 {
    () => {
        deps!();
        empty_visit ! (visit_datetime , Formatted < Datetime >) ;
    };
}

macro_402!()