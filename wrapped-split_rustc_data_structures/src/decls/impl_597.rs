macro_rules! deps {
    () => {
        MaybeTempDir!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        impl MaybeTempDir { pub fn new (dir : TempDir , keep_on_drop : bool) -> MaybeTempDir { MaybeTempDir { dir : ManuallyDrop :: new (dir) , keep : keep_on_drop } } }
    };
}

impl_597!()