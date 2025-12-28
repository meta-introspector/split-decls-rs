macro_rules! deps {
    () => {
        FileType!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl FileType { pub fn from_path (path : & std :: path :: Path) -> Self { let meta = path . symlink_metadata () ; match meta { Ok (meta) => { if meta . is_dir () { Self :: Dir } else if meta . is_file () { Self :: File } else { let target = std :: fs :: read_link (path) . ok () ; if target . is_some () { Self :: Symlink } else { Self :: Unknown } } } Err (err) => match err . kind () { std :: io :: ErrorKind :: NotFound => Self :: Missing , _ => Self :: Unknown , } , } } }
    };
}

impl_161!()