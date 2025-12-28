macro_rules! deps {
    () => {
        Result!();
        ParseStream!();
        Cursor!();
    };
}

macro_rules! StepCursor {
    () => {
        deps!();
        # [doc = " Cursor state associated with speculative parsing."] # [doc = ""] # [doc = " This type is the input of the closure provided to [`ParseStream::step`]."] # [doc = ""] # [doc = " [`ParseStream::step`]: ParseBuffer::step"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use proc_macro2::TokenTree;"] # [doc = " use syn::Result;"] # [doc = " use syn::parse::ParseStream;"] # [doc = ""] # [doc = " // This function advances the stream past the next occurrence of `@`. If"] # [doc = " // no `@` is present in the stream, the stream position is unchanged and"] # [doc = " // an error is returned."] # [doc = " fn skip_past_next_at(input: ParseStream) -> Result<()> {"] # [doc = "     input.step(|cursor| {"] # [doc = "         let mut rest = *cursor;"] # [doc = "         while let Some((tt, next)) = rest.token_tree() {"] # [doc = "             match &tt {"] # [doc = "                 TokenTree::Punct(punct) if punct.as_char() == '@' => {"] # [doc = "                     return Ok(((), next));"] # [doc = "                 }"] # [doc = "                 _ => rest = next,"] # [doc = "             }"] # [doc = "         }"] # [doc = "         Err(cursor.error(\"no `@` was found after this point\"))"] # [doc = "     })"] # [doc = " }"] # [doc = " #"] # [doc = " # fn remainder_after_skipping_past_next_at("] # [doc = " #     input: ParseStream,"] # [doc = " # ) -> Result<proc_macro2::TokenStream> {"] # [doc = " #     skip_past_next_at(input)?;"] # [doc = " #     input.parse()"] # [doc = " # }"] # [doc = " #"] # [doc = " # use syn::parse::Parser;"] # [doc = " # let remainder = remainder_after_skipping_past_next_at"] # [doc = " #     .parse_str(\"a @ b c\")"] # [doc = " #     .unwrap();"] # [doc = " # assert_eq!(remainder.to_string(), \"b c\");"] # [doc = " ```"] pub struct StepCursor < 'c , 'a > { scope : Span , cursor : Cursor < 'c > , marker : PhantomData < fn (Cursor < 'c >) -> Cursor < 'a > > , }
    };
}

StepCursor!();