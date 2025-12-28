macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! Take {
    () => {
        deps!();
        pub struct Take < R : Read > { inner : R , limit : u64 , }
    };
}

Take!();