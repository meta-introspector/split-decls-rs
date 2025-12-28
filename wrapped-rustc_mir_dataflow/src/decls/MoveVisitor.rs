macro_rules! deps {
    () => {
        BorrowedLocalsResults!();
    };
}

macro_rules! MoveVisitor {
    () => {
        deps!();
        struct MoveVisitor < 'a , 'mir , 'tcx > { borrowed_locals : & 'a mut BorrowedLocalsResults < 'mir , 'tcx > , state : & 'a mut DenseBitSet < Local > , }
    };
}

MoveVisitor!();