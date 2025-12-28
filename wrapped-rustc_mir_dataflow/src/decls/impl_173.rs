macro_rules! deps {
    () => {
        BorrowedLocalsResults!();
        MaybeRequiresStorage!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'mir , 'tcx > MaybeRequiresStorage < 'mir , 'tcx > { pub fn new (borrowed_locals : BorrowedLocalsResults < 'mir , 'tcx >) -> Self { MaybeRequiresStorage { borrowed_locals } } }
    };
}

impl_173!()