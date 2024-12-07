use nom::IResult;
use nom::character::complete::digit1;
use nom::combinator::map_res;

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}
