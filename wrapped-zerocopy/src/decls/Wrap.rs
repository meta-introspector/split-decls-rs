macro_rules! Wrap {
    () => {
        # [derive (Copy , Clone)] pub struct Wrap < Src , Dst > (pub Src , pub PhantomData < Dst >) ;
    };
}

Wrap!();