macro_rules! deps {
    () => {
        Result!();
        Error!();
        Signer!();
    };
}

macro_rules! MultipartSigner {
    () => {
        deps!();
        # [doc = " Equivalent of [`Signer`] but the message is provided in non-contiguous byte slices."] pub trait MultipartSigner < S > { # [doc = " Equivalent of [`Signer::sign()`] but the message"] # [doc = " is provided in non-contiguous byte slices."] fn multipart_sign (& self , msg : & [& [u8]]) -> S { self . try_multipart_sign (msg) . expect ("signature operation failed") } # [doc = " Equivalent of [`Signer::try_sign()`] but the"] # [doc = " message is provided in non-contiguous byte slices."] fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < S , Error > ; }
    };
}

MultipartSigner!()