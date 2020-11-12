mod macros {
    #[macro_export]
    macro_rules! set {
        ( $($x:expr),* ) => {
            {
                let mut temp_set = HashSet::new();
                $(temp_set.insert($x);)*
                temp_set
            }
        };
    }
}

