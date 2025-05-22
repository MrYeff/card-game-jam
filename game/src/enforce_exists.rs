#[macro_export]
macro_rules! enforce_exists {
    ($t: ty) => {
        panic!("{} must be set", stringify!($t)) as $t
    };
}
