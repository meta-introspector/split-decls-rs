macro_rules! deps {
    () => {
        ParseStream!();
        Cursor!();
        Parser!();
        Unexpected!();
    };
}

macro_rules! ParseBuffer {
    () => {
        deps!();
        # [doc = " Cursor position within a buffered token stream."] # [doc = ""] # [doc = " This type is more commonly used through the type alias [`ParseStream`] which"] # [doc = " is an alias for `&ParseBuffer`."] # [doc = ""] # [doc = " `ParseStream` is the input type for all parser functions in Syn. They have"] # [doc = " the signature `fn(ParseStream) -> Result<T>`."] # [doc = ""] # [doc = " ## Calling a parser function"] # [doc = ""] # [doc = " There is no public way to construct a `ParseBuffer`. Instead, if you are"] # [doc = " looking to invoke a parser function that requires `ParseStream` as input,"] # [doc = " you will need to go through one of the public parsing entry points."] # [doc = ""] # [doc = " - The [`parse_macro_input!`] macro if parsing input of a procedural macro;"] # [doc = " - One of [the `syn::parse*` functions][syn-parse]; or"] # [doc = " - A method of the [`Parser`] trait."] # [doc = ""] # [doc = " [`parse_macro_input!`]: crate::parse_macro_input!"] # [doc = " [syn-parse]: self#the-synparse-functions"] pub struct ParseBuffer < 'a > { scope : Span , cell : Cell < Cursor < 'static > > , marker : PhantomData < Cursor < 'a > > , unexpected : Cell < Option < Rc < Cell < Unexpected > > > > , }
    };
}

ParseBuffer!();