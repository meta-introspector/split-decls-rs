macro_rules! NonceGenerator {
    () => {
        # [doc = " A type to generate nonces. Store it in a static and each nonce it produces will be unique from other nonces."] # [doc = " The type parameter `T` just serves to distinguish different kinds of nonces."] pub (crate) struct NonceGenerator < T > { value : AtomicU32 , phantom : PhantomData < T > , }
    };
}

NonceGenerator!()