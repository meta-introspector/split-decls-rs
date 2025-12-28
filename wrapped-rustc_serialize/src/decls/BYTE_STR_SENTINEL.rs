macro_rules! BYTE_STR_SENTINEL {
    () => {
        # [doc = " For byte strings there are no bytes that cannot occur. Just use this value"] # [doc = " as a best-effort sentinel. There is no validation skipped so the potential"] # [doc = " for badness is lower than in the `STR_SENTINEL` case."] const BYTE_STR_SENTINEL : u8 = 0xC2 ;
    };
}

BYTE_STR_SENTINEL!()