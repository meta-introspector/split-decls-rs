macro_rules! MaybeBorrowedLocals {
    () => {
        # [doc = " A dataflow analysis that tracks whether a pointer or reference could possibly exist that points"] # [doc = " to a given local. This analysis ignores fake borrows, so it should not be used by"] # [doc = " borrowck."] # [doc = ""] # [doc = " At present, this is used as a very limited form of alias analysis. For example,"] # [doc = " `MaybeBorrowedLocals` is used to compute which locals are live during a yield expression for"] # [doc = " immovable coroutines."] # [derive (Clone)] pub struct MaybeBorrowedLocals ;
    };
}

MaybeBorrowedLocals!();