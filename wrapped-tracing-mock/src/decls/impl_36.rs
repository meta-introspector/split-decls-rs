macro_rules! deps {
    () => {
        CheckVisitor!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl CheckVisitor < '_ > { pub (crate) fn finish (self) { assert ! (self . expect . fields . is_empty () , "[{}] {}missing {}" , self . subscriber_name , self . expect , self . ctx) ; } }
    };
}

impl_36!();