//! This crate provides a [`fallthrough`] macro,
//! which allows performing a pattern match with fallthrough through the arms,
//! in the style of [C `switch`](https://en.cppreference.com/w/c/language/switch).
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, clippy::cargo, clippy::semicolon_if_nothing_returned)]

/// Accpets a match scrutinee,
/// followed by a comma-separated list of zero or more pattern match arms.
/// All arms but the first must be preceded with a `'label: `. Only the first arm
/// can acccess identifiers bound by the pattern match. Inside the match arms,
/// calling `break 'label;` will jump to the label's corresponding match arm
/// (you can only jump downwards). The list of arms can optionally be followed by a final
/// trailing label, which can be used to jump out of the fallthrough entirely.
///
/// # Example
///
/// ```
/// use fallthrough::fallthrough;
///
/// fn fall(scrutinee: u32) -> u32 {
///     let mut ret: u32 = 0;
///
///     fallthrough!(scrutinee,
///         val @ (0 | 63..) => ret = val + 7,
///         'one: 1 => ret += 8,
///         'two: 2 => ret += 9,
///         'three: 3 if true => { ret += 10; break 'end },
///         'four: 4 => ret = 42,
///         'five: 5 => { ret += 1; break 'seven },
///         'six: 6 => ret = 3,
///         'seven: _ => ret *= 2,
///         'end
///     );
///     ret
/// }
///
/// fn main() {
///     assert_eq!(fall(0), 34);
///     assert_eq!(fall(1), 27);
///     assert_eq!(fall(2), 19);
///     assert_eq!(fall(3), 10);
///     assert_eq!(fall(4), 86);
///     assert_eq!(fall(5), 2);
///     assert_eq!(fall(6), 6);
///     assert_eq!(fall(7), 0);
///     assert_eq!(fall(64), 98);
/// }
/// ```

#[macro_export]
macro_rules! fallthrough {
    ($scrutinee:expr $(,)?) => {
        match $scrutinee {}
    };
    ($scrutinee:expr, $first_pat:pat $(if $first_guard:expr)? => $first_body:expr $(, $label:lifetime $(: $pat:pat $(if $guard:expr)? => $body:expr)?)* $(,)?) => {
        $crate::fallthrough_rec!{ (match $scrutinee {
            $first_pat $(if $first_guard)? => $first_body,
            $($($pat $(if $guard)? => break $label,)?)*
        }), $($label $(: ($body))?),* }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! fallthrough_rec {
    (($($acc:tt)*),) => {$($acc)*};
    (($($acc:tt)*), $label:lifetime) => {
        $label: {
            $($acc)*
        }
    };
    (($($acc:tt)*), $label:lifetime: ($($body:tt)*) $(,$($follow:tt)*)? ) => {

        $crate::fallthrough_rec!{($label: {
                $($acc)*
            }
            $($body)*
        ), $($($follow)*)?}
    };
}
