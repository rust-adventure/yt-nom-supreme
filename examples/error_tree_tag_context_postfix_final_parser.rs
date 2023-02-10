use nom::{
    bytes::complete::take_while_m_n, sequence::tuple,
    IResult, Parser,
};
use nom_supreme::final_parser::{
    ExtractContext, Location, RecreateContext,
};
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use nom_supreme::{
    error::ErrorTree, final_parser::final_parser,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(
    input: &str,
) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(
    input: &str,
) -> IResult<&str, u8, ErrorTree<&str>> {
    take_while_m_n(2, 2, is_hex_digit)
        .context("Should be a 2 digit hex code")
        .map_res(from_hex)
        .parse(input)
}

fn hex_color(
    input: &str,
) -> IResult<&str, Color, ErrorTree<&str>> {
    tuple((hex_primary, hex_primary, hex_primary))
        .preceded_by(tag("#"))
        .parse(input)
        .map(|(input, (red, green, blue))| {
            (input, Color { red, green, blue })
        })
}

fn hex_color_final(
    input: &str,
) -> Result<Color, ErrorTree<&str>> {
    final_parser(hex_color)(input)
}
fn main() {
    assert_eq!(
        hex_color_final("#2F14DF").unwrap(),
        Color {
            red: 47,
            green: 20,
            blue: 223,
        }
    );

    let result = hex_color_final("#5");
    if let Err(
        nom_supreme::error::GenericErrorTree::Stack {
            base,
            contexts,
        },
    ) = result
    {
        dbg!(&base);
        for context in contexts {
            dbg!(Location::recreate_context(
                "#5", context.0
            ));
        }
    }

    dbg!(hex_color_final("234567"));

    hex_color_final("#12").unwrap();
}
