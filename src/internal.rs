macro_rules! impl_func_unary {
    ($(#[$attr:meta])* => $ty:ty, $name:ident, $gen_name:ident, $var:ident) => {
        $(#[$attr])*
        pub fn $name(a: $ty) -> $ty {
            $gen_name(a, &RoundMode::$var)
        }
    };
    ($(#[$attr:meta])* => $ty:ty, $name:ident, $c_name:ident) => {
        $(#[$attr])*
        pub fn $name(a: $ty, mode: &RoundMode) -> $ty {
            let mut dst: $ty = Default::default();
            match unsafe { $c_name(mode.as_c_int(), a, &mut dst) } {
                0 => dst,
                _ => panic!("fail to set/rest rounding mode"),
            }
        }
    };
}

macro_rules! impl_func_binary {
    ($(#[$attr:meta])* => $ty:ty, $name:ident , $generic_name:ident, $var:ident) => {
        $(#[$attr])*
        pub fn $name(a: $ty, b: $ty) -> $ty {
            $generic_name(a, b, &RoundMode::$var)
        }
    };
    ($(#[$attr:meta])* => $ty:ty, $name:ident, $c_name:ident) => {
        $(#[$attr])*
        pub fn $name(a: $ty, b: $ty, mode: &RoundMode) -> $ty {
            let mut dst: $ty = Default::default();
            match unsafe { $c_name(mode.as_c_int(), a, b, &mut dst) } {
                0 => dst,
                _ => panic!("fail to set/rest rounding mode"),
            }
        }
    };
}

macro_rules! impl_round_func_binary_all {
    (
        $ty:ty,
        $add:ident => $add_fn:ident,
        $sub:ident => $sub_fn:ident,
        $mul:ident => $mul_fn:ident,
        $div:ident => $div_fn:ident,
    ) => {
        impl_func_binary!(
            #[doc = concat!("Returns `a + b` as specific rounding mode.\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $add, $add_fn
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a - b` as specific rounding mode..\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $sub, $sub_fn
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a * b` as specific rounding mode.\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $mul, $mul_fn
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a / b` as specific rounding mode.\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $div, $div_fn
        );
    }
}

macro_rules! impl_non_round_func_binary_all {
    (
        $ty:ty, $mode:ident, $mode_txt:expr,
        $add:ident => $add_fn:ident,
        $sub:ident => $sub_fn:ident,
        $mul:ident => $mul_fn:ident,
        $div:ident => $div_fn:ident,
    ) => {
        impl_func_binary!(
            #[doc = concat!("Returns `a + b` as ", $mode_txt, ".\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $add, $add_fn, $mode
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a - b` as ", $mode_txt, ".\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $sub, $sub_fn, $mode
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a * b` as ", $mode_txt, ".\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $mul, $mul_fn, $mode
        );
        impl_func_binary!(
            #[doc = concat!("Returns `a / b` as ", $mode_txt, ".\n\n# Safety\n\nPanics when fail to set/rest rounding mode.")]
            #[inline]
            => $ty, $div, $div_fn, $mode
        );
    }
}

macro_rules! impl_round_binary {
    ($name:ident) => {
        #[inline]
        fn $name(self, other: Self, mode: &RoundMode) -> Self::Output {
            $name(self, other, mode)
        }
    };
}

macro_rules! impl_non_round_binary {
    ($name:ident) => {
        #[inline]
        fn $name(self, other: Self) -> Self::Output {
            $name(self, other)
        }
    };
}

pub(crate) use impl_func_binary;
pub(crate) use impl_func_unary;
pub(crate) use impl_non_round_binary;
pub(crate) use impl_non_round_func_binary_all;
pub(crate) use impl_round_binary;
pub(crate) use impl_round_func_binary_all;
