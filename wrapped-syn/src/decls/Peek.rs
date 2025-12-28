macro_rules! Peek {
    () => {
        # [doc = " Types that can be parsed by looking at just one token."] # [doc = ""] # [doc = " Use [`ParseStream::peek`] to peek one of these types in a parse stream"] # [doc = " without consuming it from the stream."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of Syn."] # [doc = ""] # [doc = " [`ParseStream::peek`]: crate::parse::ParseBuffer::peek"] pub trait Peek : Sealed { # [doc (hidden)] type Token : Token ; }
    };
}

Peek!()