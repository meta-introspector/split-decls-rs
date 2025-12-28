macro_rules! deps {
    () => {
        MaybeRequiresStorage!();
        BorrowedLocalsResults!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'mir , 'tcx > MaybeRequiresStorage < 'mir , 'tcx > { pub fn new (borrowed_locals : BorrowedLocalsResults < 'mir , 'tcx >) -> Self { MaybeRequiresStorage { borrowed_locals } } }
    };
}

impl_173!();