macro_rules! deps {
    () => {
        BorrowedLocalsResults!();
    };
}

macro_rules! MaybeRequiresStorage {
    () => {
        deps!();
        # [doc = " Dataflow analysis that determines whether each local requires storage at a"] # [doc = " given location; i.e. whether its storage can go away without being observed."] pub struct MaybeRequiresStorage < 'mir , 'tcx > { borrowed_locals : BorrowedLocalsResults < 'mir , 'tcx > , }
    };
}

MaybeRequiresStorage!()