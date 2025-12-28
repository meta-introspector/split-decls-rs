macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl From < ZipError > for io :: Error { fn from (err : ZipError) -> io :: Error { let kind = match & err { ZipError :: Io (err) => err . kind () , ZipError :: InvalidArchive (_) => io :: ErrorKind :: InvalidData , ZipError :: UnsupportedArchive (_) => io :: ErrorKind :: Unsupported , ZipError :: FileNotFound => io :: ErrorKind :: NotFound , ZipError :: InvalidPassword => io :: ErrorKind :: InvalidInput , } ; io :: Error :: new (kind , err) } }
    };
}

impl_136!()