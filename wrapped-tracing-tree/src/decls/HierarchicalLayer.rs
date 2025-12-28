macro_rules! deps {
    () => {
        FormatTime!();
        Buffers!();
        Config!();
    };
}

macro_rules! HierarchicalLayer {
    () => {
        deps!();
        # [derive (Debug)] pub struct HierarchicalLayer < W = fn () -> io :: Stderr , FT = () > where W : for < 'writer > MakeWriter < 'writer > + 'static , FT : FormatTime , { make_writer : W , bufs : Mutex < Buffers > , config : Config , timer : FT , }
    };
}

HierarchicalLayer!()