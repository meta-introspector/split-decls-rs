macro_rules! deps {
    () => {
        IdentPrinter!();
        Ident!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        # [doc = " This implementation is supposed to be used in error messages, so it's expected to be identical"] # [doc = " to printing the original identifier token written in source code (`token_to_string`),"] # [doc = " except that AST identifiers don't keep the rawness flag, so we have to guess it."] impl fmt :: Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& IdentPrinter :: new (self . name , self . guess_print_mode () , None) , f) } }
    };
}

impl_181!()