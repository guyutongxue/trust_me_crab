/// Just wrap unsafe keyword.
///
/// Trust me; everything will be SAFE.
#[macro_export]
macro_rules! safe {
    ($($x:tt)*) => {
        unsafe {
            $($x)*
        }
    };
}
