macro_rules! STR_SENTINEL {
    () => {
        # [doc = " A byte that [cannot occur in UTF8 sequences][utf8]. Used to mark the end of a string."] # [doc = " This way we can skip validation and still be relatively sure that deserialization"] # [doc = " did not desynchronize."] # [doc = ""] # [doc = " [utf8]: https://en.wikipedia.org/w/index.php?title=UTF-8&oldid=1058865525#Codepage_layout"] const STR_SENTINEL : u8 = 0xC1 ;
    };
}

STR_SENTINEL!()