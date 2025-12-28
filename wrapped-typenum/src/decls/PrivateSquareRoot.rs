macro_rules! PrivateSquareRoot {
    () => {
        pub trait PrivateSquareRoot { type Output ; }
    };
}

PrivateSquareRoot!()