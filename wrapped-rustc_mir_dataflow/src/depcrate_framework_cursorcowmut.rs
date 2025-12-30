// Generated macro for CowMut (enum)
macro_rules! Depcrate_framework_cursorCowMut {
() => {
// Module: crate::framework::cursor
// Provides: {"CowMut"}
// Dependencies: {}
# [doc = " Some `ResultsCursor`s want to own an `Analysis`, and some want to borrow an `Analysis`, either"] # [doc = " mutable or immutably. This type allows all of the above. It's similar to `Cow`, but `Cow`"] # [doc = " doesn't allow mutable borrowing."] enum CowMut < 'a , T > { BorrowedMut (& 'a mut T) , Owned (T) , }
};
}
