macro_rules! deps {
    () => {
        Filesystem!();
        FileStatus!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Filesystem { fn is_ok (& self) -> bool { if self . context . is_empty () { true } else { self . context . iter () . all (FileStatus :: is_ok) } } }
    };
}

impl_72!();