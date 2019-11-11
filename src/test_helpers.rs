#[macro_export]
macro_rules! test_parse {
    ($parser:expr, $($input:expr => $output:expr),+$(,)?) => {
        $({
            let input = $parser($input.trim())
                .map_err(|err| anyhow!("parse error {:?}", err))
                .and_then(|(remainder, v)| {
                    if remainder == "" {
                        Ok(v)
                    } else {
                        Err(anyhow!("input partially parsed, remainder: {:#?}", remainder))
                    }
                })?;
            assert_eq!(input, $output);
        })+
    };
}

#[macro_export]
macro_rules! test_part {
    ($parser: expr, $part:expr, $($input:expr => $output:expr),+$(,)?) => {
        $({
            let input = $parser($input.trim())
                .map_err(|err| anyhow!("parse error {:?}", err))
                .and_then(|(remainder, v)| {
                    if remainder == "" {
                        Ok(v)
                    } else {
                        Err(anyhow!("input partially parsed, remainder: {:#?}", remainder))
                    }
                })?;
            assert_eq!($part(input)?, $output);
        })+
    };
    ($part:expr, $($input:expr => $output:expr),+$(,)?) => {
        $({
            assert_eq!($part($input)?, $output);
        })+
    }
}
