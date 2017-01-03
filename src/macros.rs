#[macro_export]
macro_rules! try_or_false {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            warn!(err);
            return false
        }
    })
}