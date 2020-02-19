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
macro_rules! kformat {
    () => {{
        format!(concat!(file!(), ":", line!()))
    }};
    ($literal:expr) => {{
        format!(concat!(file!(), ":", line!(), "|", $literal))
    }};
    ($literal:expr, $($arg:tt)*) => {{
        format!(concat!(file!(), ":", line!(), "|", $literal), $($arg)*)
    }};
}

#[macro_export]
macro_rules! kpanic {
    () => {{
        panic!(concat!(file!(), ":", line!()))
    }};
    ($literal:expr) => {{
        panic!(concat!(file!(), ":", line!(), "|", $literal))
    }};
    ($literal:expr, $($arg:tt)*) => {{
        panic!(concat!(file!(), ":", line!(), "|", $literal), $($arg)*)
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_format() {
        println!("{}", kformat!("test"));
        println!("{}", kformat!("test {}", "argtest"));
    }
}
