macro_rules! deps {
    () => {
        MaybeBorrowedLocals!();
        TransferFunction!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl MaybeBorrowedLocals { pub (super) fn transfer_function < 'a , T > (trans : & 'a mut T) -> TransferFunction < 'a , T > { TransferFunction { trans } } }
    };
}

impl_126!()