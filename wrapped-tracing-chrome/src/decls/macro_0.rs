macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! macro_0 {
    () => {
        deps!();
        thread_local ! { static OUT : RefCell < Option < Sender < Message >>> = const { RefCell :: new (None) } ; static TID : RefCell < Option < usize >> = const { RefCell :: new (None) } ; }
    };
}

macro_0!();