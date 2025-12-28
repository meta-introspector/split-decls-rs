macro_rules! deps {
    () => {
        EcPrivateKey!();
        Result!();
    };
}

macro_rules! DecodeEcPrivateKey {
    () => {
        deps!();
        # [doc = " Parse an [`EcPrivateKey`] from a SEC1-encoded document."] pub trait DecodeEcPrivateKey : Sized { # [doc = " Deserialize SEC1 private key from ASN.1 DER-encoded data"] # [doc = " (binary format)."] fn from_sec1_der (bytes : & [u8]) -> Result < Self > ; # [doc = " Deserialize SEC1-encoded private key from PEM."] # [doc = ""] # [doc = " Keys in this format begin with the following:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN EC PRIVATE KEY-----"] # [doc = " ```"] # [cfg (feature = "pem")] fn from_sec1_pem (s : & str) -> Result < Self > { let (label , doc) = SecretDocument :: from_pem (s) ? ; EcPrivateKey :: validate_pem_label (label) ? ; Self :: from_sec1_der (doc . as_bytes ()) } # [doc = " Load SEC1 private key from an ASN.1 DER-encoded file on the local"] # [doc = " filesystem (binary format)."] # [cfg (feature = "std")] fn read_sec1_der_file (path : impl AsRef < Path >) -> Result < Self > { Self :: from_sec1_der (SecretDocument :: read_der_file (path) ? . as_bytes ()) } # [doc = " Load SEC1 private key from a PEM-encoded file on the local filesystem."] # [cfg (all (feature = "pem" , feature = "std"))] fn read_sec1_pem_file (path : impl AsRef < Path >) -> Result < Self > { let (label , doc) = SecretDocument :: read_pem_file (path) ? ; EcPrivateKey :: validate_pem_label (& label) ? ; Self :: from_sec1_der (doc . as_bytes ()) } }
    };
}

DecodeEcPrivateKey!()