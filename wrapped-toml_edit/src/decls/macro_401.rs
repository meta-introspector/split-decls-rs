macro_rules! deps {
    () => {
        Formatted!();
    };
}

macro_rules! macro_401 {
    () => {
        deps!();
        empty_visit ! (visit_boolean , Formatted < bool >) ;
    };
}

macro_401!();