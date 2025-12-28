macro_rules! deps {
    () => {
        PathError!();
        IoResultExt!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T > IoResultExt < T > for Result < T , io :: Error > { fn with_err_path < F , P > (self , path : F) -> Self where F : FnOnce () -> P , P : Into < PathBuf > , { self . map_err (| e | { io :: Error :: new (e . kind () , PathError { path : path () . into () , err : e , } ,) }) } }
    };
}

impl_22!()