macro_rules! deps {
    () => {
        ScopedKey!();
    };
}

macro_rules! scoped_thread_local {
    () => {
        deps!();
        # [doc = " The macro. See the module level documentation for the description and examples."] # [macro_export] macro_rules ! scoped_thread_local { ($ (# [$ attrs : meta]) * $ vis : vis static $ name : ident : $ ty : ty) => ($ (# [$ attrs]) * $ vis static $ name : $ crate :: ScopedKey <$ ty > = unsafe { :: std :: thread_local ! (static FOO : :: std :: cell :: Cell <* const () > = const { :: std :: cell :: Cell :: new (:: std :: ptr :: null ()) }) ; $ crate :: ScopedKey :: new (& FOO) } ;) }
    };
}

scoped_thread_local!()