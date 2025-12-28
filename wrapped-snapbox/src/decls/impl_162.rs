macro_rules! deps {
    () => {
        FileType!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl FileType { fn as_str (self) -> & 'static str { match self { Self :: Dir => "dir" , Self :: File => "file" , Self :: Symlink => "symlink" , Self :: Unknown => "unknown" , Self :: Missing => "missing" , } } }
    };
}

impl_162!()