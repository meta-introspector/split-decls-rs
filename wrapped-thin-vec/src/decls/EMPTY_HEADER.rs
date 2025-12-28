macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! EMPTY_HEADER {
    () => {
        deps!();
        # [doc = " Singleton that all empty collections share."] # [doc = " Note: can't store non-zero ZSTs, we allocate in that case. We could"] # [doc = " optimize everything to not do that (basically, make ptr == len and branch"] # [doc = " on size == 0 in every method), but it's a bunch of work for something that"] # [doc = " doesn't matter much."] # [cfg (any (not (feature = "gecko-ffi") , test , miri))] static EMPTY_HEADER : Header = Header { _len : 0 , _cap : 0 } ;
    };
}

EMPTY_HEADER!();