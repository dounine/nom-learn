fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use std::error::Error;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha0, alphanumeric0};
    use nom::IResult;

    //识别字符串
    #[test]
    fn test_tag_input() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = tag("abc")("abc123");
        let (remaing, output) =result?;
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
        Ok(())
    }
}
