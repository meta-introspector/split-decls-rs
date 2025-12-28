macro_rules! IndentedLineKind {
    () => {
        # [derive (PartialEq)] enum IndentedLineKind { Heading , Code (usize) , Note , Other (usize) , }
    };
}

IndentedLineKind!();