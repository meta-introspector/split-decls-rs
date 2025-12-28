macro_rules! deps {
    () => {
        MaybeEncrypted!();
        ZipResult!();
        GenericZipWriter!();
    };
}

macro_rules! SwitchWriterFunction {
    () => {
        deps!();
        type SwitchWriterFunction < W > = Box < dyn FnOnce (MaybeEncrypted < W >) -> ZipResult < GenericZipWriter < W > > > ;
    };
}

SwitchWriterFunction!();