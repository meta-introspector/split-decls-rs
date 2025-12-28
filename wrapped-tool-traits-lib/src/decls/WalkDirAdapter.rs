macro_rules! WalkDirAdapter {
    () => {
        pub trait WalkDirAdapter : Send + Sync { fn new (path : & Path) -> Self where Self : Sized ; fn into_iter (& self) -> Box < dyn Iterator < Item = Result < PathBuf , io :: Error > > + '_ > ; }
    };
}

WalkDirAdapter!()