use super::SimpleError;
use crate::parser::command::*;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::combinator::value;
use nom::Err;
use nom::IResult;

/// the topic that is given to HELP
pub(super) fn help_topic(input: &str) -> IResult<&str, HelpTopic, SimpleError> {
    alt((
        value(HelpTopic::General, tag_no_case("help")),
        value(HelpTopic::Size, tag_no_case("size")),
        value(HelpTopic::Px, tag_no_case("px")),
        value(HelpTopic::State, tag_no_case("state")),
    ))(input)
    .map_err(|_: Err<()>| Err::Error(SimpleError::HelpTopic(input.to_string())))
}

/// one ore more spacing characters (whitespace, tabs, …) which are discarded
pub(super) fn whitespace(input: &str) -> IResult<&str, (), SimpleError> {
    let (input, _) = multispace1(input)?;
    Ok((input, ()))
}

/// a key which can be given to STATE to specify the encoding algorithm
pub(super) fn encoding_algorithm(input: &str) -> IResult<&str, StateEncodingAlgorithm, SimpleError> {
    alt((
        value(StateEncodingAlgorithm::Rgb64, tag_no_case("rgb64")),
        value(StateEncodingAlgorithm::Rgba64, tag_no_case("rgba64")),
    ))(input)
    .map_err(|_: Err<()>| Err::Error(SimpleError::StateEncodingAlgorithm(input.to_string())))
}

#[derive(Debug, Copy, Clone)]
pub(super) enum PrimaryCommand {
    Help,
    Size,
    Px,
    State,
}

/// the first word of a pixelflut command like HELP, SIZE, PX, etc.
pub(super) fn primary_command(input: &str) -> IResult<&str, PrimaryCommand, SimpleError> {
    alt((
        value(PrimaryCommand::Help, tag_no_case("help")),
        value(PrimaryCommand::Size, tag_no_case("size")),
        value(PrimaryCommand::Px, tag_no_case("px")),
        value(PrimaryCommand::State, tag_no_case("state")),
    ))(input)
    .map_err(|_: Err<()>| Err::Error(SimpleError::PrimaryCommand(input.to_string())))
}
