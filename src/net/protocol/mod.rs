//! Definitions for the network protocol

mod compliant_parser;
mod dtypes;

pub use dtypes::*;

pub use compliant_parser::CompliantParser;
use thiserror::Error;

/// Trait for response parser implementations
pub trait RequestParser {
    /// Parse a single pixelflut request from a byte slice
    fn parse_request_bin(&self, line: &[u8]) -> Result<Request, ParseErr>;
}

/// Trait for request parser implementations
pub trait ResponseParser {
    /// Parse a single pixelflut response from a byte slice
    fn parse_response_bin(&self, line: &[u8]) -> Result<Response, ParseErr>;
}

/// Errors that can occur while parsing an input buffer
#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum ParseErr {
    /// The passed pixelflut command is unknown
    #[error("Unknown Command")]
    UnknownCommand,
    /// The passed pixelflut command is known but its invocation was invalid
    #[error("Invalid Command Invocation")]
    InvalidCommand,
}

#[cfg(test)]
mod test {
    use std::hint::black_box;

    use ::test::Bencher;

    use crate::pixmap::Color;

    use super::*;

    pub(super) fn gen_test_parse_commands(parser: impl RequestParser + Clone) {
        fn run_test(parser: impl RequestParser, line: &str, res: Request) {
            let req = parser.parse_request_bin(line.as_bytes());
            assert_eq!(req, Ok(res), "{:06x?} != Ok({:06x?})", req, res);
        }

        run_test(parser.clone(), "HELP", Request::Help(HelpTopic::General));
        run_test(parser.clone(), "SIZE", Request::GetSize);
        run_test(
            parser.clone(),
            "PX 42 128 AABBCC",
            Request::SetPixel {
                x: 42,
                y: 128,
                color: Color::from((0xAA, 0xBB, 0xCC)),
            },
        );
        run_test(
            parser.clone(),
            "PX 0 0 AABBCC",
            Request::SetPixel {
                x: 0,
                y: 0,
                color: Color::from((0xAA, 0xBB, 0xCC)),
            },
        );
    }

    pub(super) fn gen_bench_parse_get_pixel(parser: impl RequestParser, b: &mut Bencher) {
        let cmd = black_box("PX 17 7632").as_bytes();
        b.iter(move || parser.parse_request_bin(cmd).unwrap());
    }

    pub(super) fn gen_bench_parse_set_pixel(parser: impl RequestParser, b: &mut Bencher) {
        let cmd = "PX 17 7632 12FBA5".as_bytes();
        b.iter(move || parser.parse_request_bin(black_box(cmd)).unwrap());
    }

    pub(super) fn gen_bench_parse_size(parser: impl RequestParser, b: &mut Bencher) {
        let cmd = "SIZE".as_bytes();
        b.iter(move || parser.parse_request_bin(black_box(cmd)).unwrap());
    }
}
