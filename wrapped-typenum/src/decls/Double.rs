macro_rules! deps {
    () => {
        B1!();
        Shleft!();
    };
}

macro_rules! Double {
    () => {
        deps!();
        # [doc = " Alias to make it easy to multiply by 2. `Double<A> = Shleft<A, B1>`"] pub type Double < A > = Shleft < A , crate :: bit :: B1 > ;
    };
}

Double!()