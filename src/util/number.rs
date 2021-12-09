macro_rules! parse_int_impl {
    ($($t:ty, $name: ident)*) => {$(
        #[allow(unused)]
        pub fn $name(input: &str) -> Result<$t, String> {
            input.to_string().parse().map_err(|e| format!("{}", e))
        }
    )*}
}

parse_int_impl! {
    u8, parse_u8
    u16, parse_u16
    u32, parse_u32
    u64, parse_u64
    u128, parse_u128
    usize, parse_usize
    i8, parse_i8
    i16, parse_i16
    i32, parse_i32
    i64, parse_i64
    i128, parse_i128
    isize, parse_isize
}
