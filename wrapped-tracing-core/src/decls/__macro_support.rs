macro_rules! __macro_support {
    () => {
        # [doc (hidden)] pub mod __macro_support { pub use core :: { file , line , module_path , option :: Option } ; }
    };
}

__macro_support!()