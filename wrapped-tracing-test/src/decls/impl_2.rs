macro_rules! deps {
    () => {
        PollN!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl PollN < () , () > { pub fn new_ok (finish_at : usize) -> Self { Self { and_return : Some (Ok (())) , finish_at , polls : 0 , } } pub fn new_err (finish_at : usize) -> Self { Self { and_return : Some (Err (())) , finish_at , polls : 0 , } } }
    };
}

impl_2!();