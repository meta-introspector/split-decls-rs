macro_rules! deps {
    () => {
        UnsafeCell!();
    };
}

macro_rules! cell {
    () => {
        deps!();
        pub (crate) mod cell { pub (crate) use super :: unsafe_cell :: UnsafeCell ; }
    };
}

cell!();