macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! Parse {
    () => {
        deps!();
        # [doc = " `Parse` is the result of the parsing: a syntax tree and a collection of"] # [doc = " errors."] # [doc = ""] # [doc = " Note that we always produce a syntax tree, even for completely invalid"] # [doc = " files."] # [derive (Debug , PartialEq , Eq)] pub struct Parse < T > { green : Option < GreenNode > , errors : Option < Arc < [SyntaxError] > > , _ty : PhantomData < fn () -> T > , }
    };
}

Parse!()