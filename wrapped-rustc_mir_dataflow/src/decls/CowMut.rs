macro_rules! deps {
    () => {
        Analysis!();
    };
}

macro_rules! CowMut {
    () => {
        deps!();
        # [doc = " Some `ResultsCursor`s want to own an `Analysis`, and some want to borrow an `Analysis`, either"] # [doc = " mutable or immutably. This type allows all of the above. It's similar to `Cow`, but `Cow`"] # [doc = " doesn't allow mutable borrowing."] enum CowMut < 'a , T > { BorrowedMut (& 'a mut T) , Owned (T) , }
    };
}

CowMut!()