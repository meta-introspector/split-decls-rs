macro_rules! deps {
    () => {
        ParseBuffer!();
        Input!();
    };
}

macro_rules! ParseStream {
    () => {
        deps!();
        # [doc = " Input to a Syn parser function."] # [doc = ""] # [doc = " See the methods of this type under the documentation of [`ParseBuffer`]. For"] # [doc = " an overview of parsing in Syn, refer to the [module documentation]."] # [doc = ""] # [doc = " [module documentation]: self"] pub type ParseStream < 'a > = & 'a ParseBuffer < 'a > ;
    };
}

ParseStream!()