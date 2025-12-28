macro_rules! deps {
    () => {
        SourceFileHashAlgorithm!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl Display for SourceFileHashAlgorithm { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Self :: Md5 => "md5" , Self :: Sha1 => "sha1" , Self :: Sha256 => "sha256" , Self :: Blake3 => "blake3" , }) } }
    };
}

impl_298!();