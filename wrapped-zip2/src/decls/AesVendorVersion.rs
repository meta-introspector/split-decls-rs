macro_rules! AesVendorVersion {
    () => {
        # [doc = " The encryption specification used to encrypt a file with AES."] # [doc = ""] # [doc = " According to the [specification](https://www.winzip.com/win/en/aes_info.html#winzip11) AE-2"] # [doc = " does not make use of the CRC check."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u16)] pub enum AesVendorVersion { Ae1 = 0x0001 , Ae2 = 0x0002 , }
    };
}

AesVendorVersion!()