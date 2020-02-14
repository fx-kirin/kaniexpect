#[macro_export]
macro_rules! expect {
    ($option:expr) => {{
        $option.expect(concat!(file!(), ":", line!()))
    }};
    ($option:expr, $literal:expr) => {{
        $option.expect(concat!(file!(), ":", line!(), "|", $literal))
    }};
}

#[macro_export]
macro_rules! kaniformat {
    ($option:expr) => {{
        format!(concat!(file!(), ":", line!()))
    }};
    ($option:expr, $literal:expr) => {{
        format!(concat!(file!(), ":", line!(), "|", $literal))
    }};
}
