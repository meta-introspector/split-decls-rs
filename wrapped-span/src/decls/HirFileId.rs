macro_rules! deps {
    () => {
        MacroCallId!();
    };
}

macro_rules! HirFileId {
    () => {
        deps!();
        # [doc = " Input to the analyzer is a set of files, where each file is identified by"] # [doc = " `FileId` and contains source code. However, another source of source code in"] # [doc = " Rust are macros: each macro can be thought of as producing a \"temporary"] # [doc = " file\". To assign an id to such a file, we use the id of the macro call that"] # [doc = " produced the file. So, a `HirFileId` is either a `FileId` (source code"] # [doc = " written by user), or a `MacroCallId` (source code produced by macro)."] # [doc = ""] # [doc = " What is a `MacroCallId`? Simplifying, it's a `HirFileId` of a file"] # [doc = " containing the call plus the offset of the macro call in the file. Note that"] # [doc = " this is a recursive definition! However, the size_of of `HirFileId` is"] # [doc = " finite (because everything bottoms out at the real `FileId`) and small"] # [doc = " (`MacroCallId` uses the location interning. You can check details here:"] # [doc = " <https://en.wikipedia.org/wiki/String_interning>)."] # [doc = ""] # [doc = " Internally this holds a `salsa::Id`, but we cannot use this definition here"] # [doc = " as it references things from base-db and hir-expand."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct HirFileId (pub salsa :: Id) ;
    };
}

HirFileId!()