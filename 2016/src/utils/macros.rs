mod map {
    #[macro_export]
    macro_rules! map {
        ($key:ty, $val:ty) => {
            {
                let map: std::collections::HashMap<$key, $val> = std::collections::HashMap::new();
                map
            }
        };
        ($($key:expr => $val:expr),*) => {
            {
                let mut map = std::collections::HashMap::new();
                $( map.insert($key, $val); )*
                map
            }
        };
    }
}