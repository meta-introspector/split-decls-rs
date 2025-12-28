macro_rules! deps {
    () => {
        InflateStream!();
        ReturnCode!();
    };
}

macro_rules! get_header {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee:"] # [doc = ""] # [doc = " * If `head` is `Some`:"] # [doc = "     - If `head.extra` is not NULL, it must be writable for at least `head.extra_max` bytes"] # [doc = "     - if `head.name` is not NULL, it must be writable for at least `head.name_max` bytes"] # [doc = "     - if `head.comment` is not NULL, it must be writable for at least `head.comm_max` bytes"] pub unsafe fn get_header < 'a > (stream : & mut InflateStream < 'a > , head : Option < & 'a mut gz_header > ,) -> ReturnCode { if (stream . state . wrap & 2) == 0 { return ReturnCode :: StreamError ; } stream . state . head = head . map (| head | { head . done = 0 ; head }) ; ReturnCode :: Ok }
    };
}

get_header!();