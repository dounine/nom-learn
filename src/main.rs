fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub enum Json<'a> {
    Null,
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json<'a>>),
    Object(Vec<(&'a str, Json<'a>)>),
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use nom::branch::alt;
    use nom::bytes::complete::{escaped, is_not, tag, take, take_till, take_until, take_while, take_while1, take_while_m_n};
    use nom::character::complete::{alpha0, alphanumeric0, alphanumeric1, char, digit0, digit1, i32, multispace0, none_of, one_of, space1};
    use nom::combinator::{cut, map, map_res, recognize, value};
    use nom::{IResult, Parser};
    use nom::error::ParseError;
    use nom::multi::{length_count, many0, many1};
    use nom::sequence::{delimited, preceded, separated_pair, terminated, Tuple};
    use crate::Json;

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

    fn hex_primary(input: &str) -> IResult<&str, &str> {
        map_res(
            take_while_m_n(1, 1, |c| true),
            |s: &str| Ok(s) as Result<&str, std::num::ParseIntError>,
        )(input)
    }

    #[test]
    fn test_parse() -> Result<(), Box<dyn Error>> {
        let (remaing, (a, b, c)) = (hex_primary, hex_primary, hex_primary).parse("abc")?;
        assert_eq!(remaing, "");
        assert_eq!(a, "a");
        assert_eq!(b, "b");
        assert_eq!(c, "c");
        Ok(())
    }

    #[test]
    fn test_recongnize() -> Result<(), Box<dyn Error>> {
        let result: IResult<&str, &str> = preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(
                many1(
                    terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
                )
            ),
        )("0x9c9c9c");
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "9c9c9c");
        Ok(())
    }

    #[test]
    fn test_number() -> Result<(), Box<dyn Error>> {
        let num = tag("123");
        let result = num("123abc") as IResult<&str, &str>;
        let (remaing, output) = result?;
        dbg!(remaing);
        assert_eq!(remaing, "abc");
        assert_eq!(output, "123");
        Ok(())
    }

    #[test]
    fn test_not() -> Result<(), Box<dyn Error>> {
        let result = terminated(is_not("岁"), tag("岁"))("18岁") as IResult<&str, &str>;
        let (remaing, output) = result?;
        assert_eq!(remaing, "");
        assert_eq!(output, "18");
        Ok(())
    }

    #[test]
    fn test_string_parser() -> Result<(), Box<dyn Error>> {
        let str = r#""name":"lake""#;
        let mut string_inner = delimited(char('"'), take_until("\""), char('"'));
        let result: IResult<&str, &str> = string_inner(str);
        let (remaing, output) = result?;
        assert_eq!(output, "name");
        Ok(())
    }

    fn parse_boolean(input: &str) -> IResult<&str, Json> {
        alt((value(Json::Boolean(true), tag("true")), value(Json::Boolean(false), tag("false"))))(input)
    }

    fn parse_str(i: &str) -> IResult<&str, &str> {
        escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
    }

    fn parse_string(input: &str) -> IResult<&str, &str> {
        // escaped(alphanumeric1, '\\', one_of("\"n\\"))(input)
        delimited(
            char('"'),
            take_until("\""),
            char('"'),
        )(input)
    }

    #[test]
    fn test_parse_string() -> Result<(), Box<dyn Error>> {
        let input = r#"He said, \"Hello!\""#;
        let mut resource: IResult<&str, &str> =
            escaped(
                none_of(r#"\""#),
                '\\',
                one_of(r#"\"nrt"#),
            )(input);
        let (remaing, output) = resource?;
        assert_eq!(remaing, "");
        assert_eq!(output, input);
        Ok(())
    }

    fn parse_number(input: &str) -> IResult<&str, Json> {
        map_res(
            recognize(
                many1(
                    one_of("0123456789.")
                )
            ),
            |s: &str| {
                s.parse::<f64>()
                    .map(Json::Number)
            },
        )(input)
    }

    #[test]
    fn test_parse_number() -> Result<(), Box<dyn Error>> {
        let (remaing, output) = parse_number("12345.6")?;
        assert_eq!(remaing, "");
        Ok(())
    }

    fn parse_null(input: &str) -> IResult<&str, Json> {
        value(Json::Null, tag("null"))(input)
    }
}
