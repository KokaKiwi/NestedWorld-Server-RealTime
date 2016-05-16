// Helper macros for creating MessagePack values

#[macro_export]
macro_rules! rmp_array {
    [$($item:expr),*] => {
        {
            use $crate::net::msg::utils::rmp::IntoValue;
            vec![$($item.into_value()),*].into_value()
        }
    };
    [$($item:expr),*,] => {
        rmp_array![$($item),*]
    };
}

#[macro_export]
macro_rules! rmp_map {
    [$($key:expr => $value:expr),*] => {
        {
            use $crate::net::msg::utils::rmp::IntoValue;
            vec![$(($key.into_value(), $value.into_value())),*].into_value()
        }
    };
    [$($key:expr => $value:expr),*,] => {
        rmp_map[$($key => $value),*]
    };
}

