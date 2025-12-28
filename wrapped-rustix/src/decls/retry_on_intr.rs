macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! retry_on_intr {
    () => {
        deps!();
        # [doc = " Call `f` until it either succeeds or fails other than [`Errno::INTR`]."] # [inline] pub fn retry_on_intr < T , F : FnMut () -> Result < T > > (mut f : F) -> Result < T > { loop { match f () { Err (Errno :: INTR) => () , result => return result , } } }
    };
}

retry_on_intr!();