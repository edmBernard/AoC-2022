
// If using `use paste::paste` in main we can juste use `$crate::paste!` below
// but I don't want that main have to import test_helper internal module.
// Here we expose paste crate to every one to be able to use it from root crate given by `$crate`
// and use the full path of the macro `$crate::test_helper::paste::paste!`
// I don't find another way to better do that.
pub extern crate paste;

macro_rules! add_test {
    ($($name:ident: $func:ident, $filename:expr, $value:expr;)*) => {
    $(
        $crate::test_helper::paste::paste! {
            #[test]
            fn [<$name $func>]() -> Result<(), std::io::Error> {
                assert_eq!($value, $func(Path::new($filename))?);
                Ok(())
            }
        }
    )*
    }
}

pub(crate) use add_test;
