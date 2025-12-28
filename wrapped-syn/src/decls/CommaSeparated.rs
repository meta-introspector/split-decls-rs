macro_rules! CommaSeparated {
    () => {
        struct CommaSeparated < 'a > (& 'a [& 'a str]) ;
    };
}

CommaSeparated!();