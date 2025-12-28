macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl Error for ZipError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { Self :: Io (e) => Some (e) , Self :: InvalidArchive (_) | Self :: UnsupportedArchive (_) | Self :: FileNotFound | Self :: InvalidPassword => None , } } }
    };
}

impl_135!()