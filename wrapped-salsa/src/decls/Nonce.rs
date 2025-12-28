macro_rules! Nonce {
    () => {
        # [doc = " A \"nonce\" is a value that gets created exactly once."] # [doc = " We use it to mark the database storage so we can be sure we're seeing the same database."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct Nonce < T > (NonZeroU32 , PhantomData < T >) ;
    };
}

Nonce!()