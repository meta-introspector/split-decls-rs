macro_rules! MaybeTempDir {
    () => {
        # [doc = " This is used to avoid TempDir being dropped on error paths unintentionally."] # [derive (Debug)] pub struct MaybeTempDir { dir : ManuallyDrop < TempDir > , keep : bool , }
    };
}

MaybeTempDir!()