macro_rules! ScopedKey {
    () => {
        # [doc = " Type representing a thread local storage key corresponding to a reference"] # [doc = " to the type parameter `T`."] # [doc = ""] # [doc = " Keys are statically allocated and can contain a reference to an instance of"] # [doc = " type `T` scoped to a particular lifetime. Keys provides two methods, `set`"] # [doc = " and `with`, both of which currently use closures to control the scope of"] # [doc = " their contents."] pub struct ScopedKey < T > { inner : & 'static LocalKey < Cell < * const () > > , _marker : marker :: PhantomData < T > , }
    };
}

ScopedKey!()