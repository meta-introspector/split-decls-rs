macro_rules! Var {
    () => {
        pub struct Var < 'a , T : ? Sized > (pub & 'a T) ;
    };
}

Var!();