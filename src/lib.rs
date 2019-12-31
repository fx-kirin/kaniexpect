#[macro_export]
macro_rules! expect {
    ($option:expr) => {{
        $option.expect(concat!(file!(), ":", line!()))
    }};
    ($option:expr, $literal:expr) => {{
        $option.expect(concat!(file!(), ":", line!(), "|", $literal))
    }};
}
