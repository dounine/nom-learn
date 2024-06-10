fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use std::error::Error;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take, take_until, take_while};
    use nom::character::complete::{alpha0, alphanumeric0, alphanumeric1, char, digit0, digit1, i32, space1};
    use nom::combinator::map_res;
    use nom::IResult;
    use nom::multi::length_count;
    use nom::sequence::{delimited, preceded, separated_pair, terminated};

    //识别字符串
    #[test]
    fn test_tag_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = tag("abc")("abc123");
        let (remaing, output) = result?;
        assert_eq!(remaing, "123");
        assert_eq!(output, "abc");
        Ok(())
    }

    //识别零个或多个小写和大写字母：/[a-zA-Z]/
    #[test]
    fn test_alpha0_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = alpha0("abc123");
        let (remaing, output) = result?;
        assert_eq!(remaing, "123");
        assert_eq!(output, "abc");
        Ok(())
    }

    //识别零个或多个数字和字母：/[0-9a-zA-Z]/
    #[test]
    fn test_alphanumberic0_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = alphanumeric0("abc123");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "abc123");

        let result: IResult<&str, &str> = alphanumeric0(".");
        let (remaing, output) = result?;
        assert_eq!(remaing, ".");
        assert_eq!(output, "");
        Ok(())
    }

    #[test]
    fn test_alphanumberic1_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = alphanumeric1("a.");
        let (remaing, output) = result?;
        assert_eq!(remaing, ".");
        assert_eq!(output, "a");
        Ok(())
    }

    #[test]
    fn test_digit0_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = digit0("123abc");
        let (remaing, output) = result?;
        assert_eq!(remaing, "abc");
        assert_eq!(output, "123");

        let result: IResult<&str, &str> = digit0("abc");
        let (remaing, output) = result?;
        assert_eq!(remaing, "abc");
        assert_eq!(output, "");
        Ok(())
    }

    #[test]
    fn test_alt_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = alt((tag("abc"), tag("def"), tag("abd")))("abd123");
        let (remaing, output) = result?;
        assert_eq!(remaing, "123");
        assert_eq!(output, "abd");
        Ok(())
    }

    #[test]
    fn test_delimited_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = delimited(char('('), alphanumeric0, char(')'))("(abc123)");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "abc123");
        Ok(())
    }

    #[test]
    fn test_preceded_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = preceded(tag("数字："), digit1)("数字：1");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "1");
        Ok(())
    }

    #[test]
    fn test_terminated_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = terminated(digit1, tag("天"))("365天");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "365");
        Ok(())
    }

    #[test]
    fn test_i32_pair_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, (i32, i32)> = delimited(
            char('('),
            separated_pair(
                i32,
                alt(
                    (tag(","), space1)
                ),
                i32,
            ),
            char(')'),
        )("(1 2)");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, (1, 2));
        Ok(())
    }

    #[test]
    fn test_take_input() -> Result<(), Box<dyn Error>> {
        let input = "hello world. hi";
        let result: IResult<&str, &str> = terminated(take_until("."), take_while(|c| c == '.' || c == ' '))(input);
        let (remaing, output) = result?;
        assert_eq!(remaing, "hi");
        assert_eq!(output, "hello world");
        Ok(())
    }
}
