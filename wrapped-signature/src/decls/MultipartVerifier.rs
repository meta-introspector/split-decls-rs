macro_rules! deps {
    () => {
        Error!();
        Result!();
        Verifier!();
    };
}

macro_rules! MultipartVerifier {
    () => {
        deps!();
        # [doc = " Equivalent of [`Verifier`] but the message is provided in non-contiguous byte slices."] pub trait MultipartVerifier < S > { # [doc = " Equivalent of [`Verifier::verify()`] but the"] # [doc = " message is provided in non-contiguous byte slices."] fn multipart_verify (& self , msg : & [& [u8]] , signature : & S) -> Result < () , Error > ; }
    };
}

MultipartVerifier!();