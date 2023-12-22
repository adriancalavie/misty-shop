#[macro_export]
macro_rules! json_string {
    ($($json:tt)+) => {
        json!($($json)+).to_string()
    };
}
