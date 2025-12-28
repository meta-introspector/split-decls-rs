macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! LogVisitor {
    () => {
        deps!();
        struct LogVisitor < 'a > { target : Option < & 'a str > , module_path : Option < & 'a str > , file : Option < & 'a str > , line : Option < u64 > , fields : & 'static Fields , }
    };
}

LogVisitor!()