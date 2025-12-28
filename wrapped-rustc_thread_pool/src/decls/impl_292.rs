macro_rules! deps {
    () => {
        WorkerLocal!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < T > WorkerLocal < Vec < T > > { # [doc = " Joins the elements of all the worker locals into one Vec"] pub fn join (self) -> Vec < T > { self . into_inner () . into_iter () . flat_map (| v | v) . collect () } }
    };
}

impl_292!();