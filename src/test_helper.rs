
// If using `use paste::paste` in main we can juste use `$crate::paste!` below
// but I don't want that main have to import test_helper internal module.
// Here we expose paste crate to every one to be able to use it from root crate given by `$crate`
// and use the full path of the macro `$crate::test_helper::paste::paste!`
// I don't find another way to better do that.
pub extern crate paste;

/// Macro to add test given an input filename and expected value for part1 and part2
/// # Example
/// ```
///   add_test!(
///     test1:  day01, "data/day01_test1.txt", [7, 5];
///     test2:  day01, "data/day01_test1.txt", [7, 5];
///   );
/// ```
macro_rules! add_test {
    ($($name:ident: $func:ident, $filename:expr, $value:expr;)*) => {
    $(
        $crate::test_helper::paste::paste! {
            #[test]
            fn [<$name $func>]() -> $crate::Result<()> {
                assert_eq!($func(Path::new($filename))?, $value);
                Ok(())
            }
        }
    )*
    }
}

pub(crate) use add_test;
