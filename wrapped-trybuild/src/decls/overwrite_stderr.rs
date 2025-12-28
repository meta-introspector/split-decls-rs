macro_rules! overwrite_stderr {
    () => {
        pub (crate) fn overwrite_stderr (stderr_path : & Path , stderr : & str) { let stderr_path = stderr_path . to_string_lossy () ; term :: bold_color (Yellow) ; println ! ("wip") ; println ! () ; print ! ("NOTE") ; term :: reset () ; println ! (": writing the following output to `{}`." , stderr_path) ; snippet (Yellow , stderr) ; println ! () ; }
    };
}

overwrite_stderr!()