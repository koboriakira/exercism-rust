#[macro_export]
macro_rules! hashmap {
    (,) => {{}};
    ( $( $key:expr => $value:expr ),* ) => {{
        let mut result = HashMap::new();
        $(result.insert($key, $value);)*
        result
    }};
    ( $( $key:expr => $value:expr),*,) => {{
        let mut result = HashMap::new();
        $(result.insert($key, $value);)*
        result
    }};
}
