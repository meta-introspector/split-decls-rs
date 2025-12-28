macro_rules! deps {
    () => {
        TokenBuffer!();
        Entry!();
    };
}

macro_rules! Cursor {
    () => {
        deps!();
        # [doc = " A cheaply copyable cursor into a `TokenBuffer`."] # [doc = ""] # [doc = " This cursor holds a shared reference into the immutable data which is used"] # [doc = " internally to represent a `TokenStream`, and can be efficiently manipulated"] # [doc = " and copied around."] # [doc = ""] # [doc = " An empty `Cursor` can be created directly, or one may create a `TokenBuffer`"] # [doc = " object and get a cursor to its first token with `begin()`."] pub struct Cursor < 'a > { ptr : * const Entry , scope : * const Entry , marker : PhantomData < & 'a Entry > , }
    };
}

Cursor!();