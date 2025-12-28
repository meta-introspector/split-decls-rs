macro_rules! Precedence {
    () => {
        pub (crate) enum Precedence { Jump , Assign , Range , Or , And , # [cfg (feature = "printing")] Let , Compare , BitOr , BitXor , BitAnd , Shift , Sum , Product , Cast , # [cfg (feature = "printing")] Prefix , # [cfg (feature = "printing")] Unambiguous , }
    };
}

Precedence!()