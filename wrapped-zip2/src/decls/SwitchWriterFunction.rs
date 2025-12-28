macro_rules! deps {
    () => {
        ZipResult!();
        GenericZipWriter!();
        MaybeEncrypted!();
    };
}

macro_rules! SwitchWriterFunction {
    () => {
        deps!();
        type SwitchWriterFunction < W > = Box < dyn FnOnce (MaybeEncrypted < W >) -> ZipResult < GenericZipWriter < W > > > ;
    };
}

SwitchWriterFunction!()