macro_rules! UnsupportedPlatformError {
    () => {
        # [doc = " The error type returned on unsupported platforms."] # [doc = ""] # [doc = " On unsupported platforms, all operations will fail with an `io::Error` with"] # [doc = " a kind `io::ErrorKind::Unsupported` and an `UnsupportedPlatformError` error as the inner error."] # [doc = " While you *could* check the inner error, it's probably simpler just to check"] # [doc = " `xattr::SUPPORTED_PLATFORM`."] # [doc = ""] # [doc = " This error mostly exists for pretty error messages."] # [derive (Copy , Clone , Debug)] pub struct UnsupportedPlatformError ;
    };
}

UnsupportedPlatformError!();