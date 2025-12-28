macro_rules! deps {
    () => {
        RealFileName!();
    };
}

macro_rules! FileName {
    () => {
        deps!();
        # [doc = " Differentiates between real files and common virtual files."] # [derive (Debug , Eq , PartialEq , Clone , Ord , PartialOrd , Hash , Decodable , Encodable)] pub enum FileName { Real (RealFileName) , # [doc = " Strings provided as `--cfg [cfgspec]`."] CfgSpec (Hash64) , # [doc = " Command line."] Anon (Hash64) , # [doc = " Hack in `src/librustc_ast/parse.rs`."] MacroExpansion (Hash64) , ProcMacroSourceCode (Hash64) , # [doc = " Strings provided as crate attributes in the CLI."] CliCrateAttr (Hash64) , # [doc = " Custom sources for explicit parser calls from plugins and drivers."] Custom (String) , DocTest (PathBuf , isize) , # [doc = " Post-substitution inline assembly from LLVM."] InlineAsm (Hash64) , }
    };
}

FileName!()