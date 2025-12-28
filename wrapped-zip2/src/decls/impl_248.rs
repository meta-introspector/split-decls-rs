macro_rules! deps {
    () => {
        FileOptions!();
        DateTime!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < T : FileOptionExtension > Default for FileOptions < '_ , T > { # [doc = " Construct a new FileOptions object"] fn default () -> Self { Self { compression_method : Default :: default () , compression_level : None , last_modified_time : DateTime :: default_for_write () , permissions : None , large_file : false , encrypt_with : None , extended_options : T :: default () , alignment : 1 , # [cfg (feature = "deflate-zopfli")] zopfli_buffer_size : Some (1 << 15) , # [cfg (feature = "aes-crypto")] aes_mode : None , } } }
    };
}

impl_248!()