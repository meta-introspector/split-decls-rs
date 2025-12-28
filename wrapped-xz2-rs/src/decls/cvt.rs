macro_rules! deps {
    () => {
        Status!();
        Error!();
    };
}

macro_rules! cvt {
    () => {
        deps!();
        fn cvt (rc : lzma_sys :: lzma_ret) -> Result < Status , Error > { match rc { lzma_sys :: LZMA_OK => Ok (Status :: Ok) , lzma_sys :: LZMA_STREAM_END => Ok (Status :: StreamEnd) , lzma_sys :: LZMA_NO_CHECK => Err (Error :: NoCheck) , lzma_sys :: LZMA_UNSUPPORTED_CHECK => Err (Error :: UnsupportedCheck) , lzma_sys :: LZMA_GET_CHECK => Ok (Status :: GetCheck) , lzma_sys :: LZMA_MEM_ERROR => Err (Error :: Mem) , lzma_sys :: LZMA_MEMLIMIT_ERROR => Err (Error :: MemLimit) , lzma_sys :: LZMA_FORMAT_ERROR => Err (Error :: Format) , lzma_sys :: LZMA_OPTIONS_ERROR => Err (Error :: Options) , lzma_sys :: LZMA_DATA_ERROR => Err (Error :: Data) , lzma_sys :: LZMA_BUF_ERROR => Ok (Status :: MemNeeded) , lzma_sys :: LZMA_PROG_ERROR => Err (Error :: Program) , c => panic ! ("unknown return code: {}" , c) , } }
    };
}

cvt!();