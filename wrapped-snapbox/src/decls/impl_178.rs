macro_rules! deps {
    () => {
        Walk!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl Walk { pub fn new (path : & std :: path :: Path) -> Self { Self { inner : walkdir :: WalkDir :: new (path) . into_iter () , } } }
    };
}

impl_178!();