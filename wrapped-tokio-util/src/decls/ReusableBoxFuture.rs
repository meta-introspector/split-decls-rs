macro_rules! ReusableBoxFuture {
    () => {
        # [doc = " A reusable `Pin<Box<dyn Future<Output = T> + Send + 'a>>`."] # [doc = ""] # [doc = " This type lets you replace the future stored in the box without"] # [doc = " reallocating when the size and alignment permits this."] pub struct ReusableBoxFuture < 'a , T > { boxed : Pin < Box < dyn Future < Output = T > + Send + 'a > > , }
    };
}

ReusableBoxFuture!();