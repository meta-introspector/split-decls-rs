macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! JsonVisitor {
    () => {
        deps!();
        struct JsonVisitor < 'a > { object : & 'a mut Object , }
    };
}

JsonVisitor!()