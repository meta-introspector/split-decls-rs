macro_rules! SynToken {
    () => {
        # [derive (Debug)] enum SynToken < S > { Ordinary (SyntaxToken) , Punct { token : SyntaxToken , offset : usize } , Leaf (tt :: Leaf < S >) , }
    };
}

SynToken!()