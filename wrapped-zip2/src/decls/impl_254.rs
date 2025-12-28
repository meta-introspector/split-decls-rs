macro_rules! deps {
    () => {
        StreamWriter!();
        MaybeEncrypted!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl < W : Write > ZipWriter < StreamWriter < W > > { # [doc = " Creates a writer that doesn't require the inner writer to implement [Seek], but where"] # [doc = " operations that would overwrite previously-written bytes or cause subsequent operations to"] # [doc = " do so (such as `abort_file`) will always return an error."] pub fn new_stream (inner : W) -> ZipWriter < StreamWriter < W > > { ZipWriter { inner : Storer (MaybeEncrypted :: Unencrypted (StreamWriter :: new (inner))) , files : IndexMap :: new () , stats : Default :: default () , writing_to_file : false , writing_raw : false , comment : Box :: new ([]) , zip64_comment : None , flush_on_finish_file : false , seek_possible : false , } } }
    };
}

impl_254!();