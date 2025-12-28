macro_rules! deps {
    () => {
        ResultsCursor!();
        MaybeBorrowedLocals!();
    };
}

macro_rules! BorrowedLocalsResults {
    () => {
        deps!();
        type BorrowedLocalsResults < 'mir , 'tcx > = ResultsCursor < 'mir , 'tcx , MaybeBorrowedLocals > ;
    };
}

BorrowedLocalsResults!()