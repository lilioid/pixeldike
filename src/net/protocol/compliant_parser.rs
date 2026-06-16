//! A pixelflut request parser implementation that is fully compliant to the wire protocol

use crate::net::protocol::{HelpTopic, ParseErr, Request, RequestParser, Response, ResponseParser};
use crate::pixmap::Color;

/// A pixelflut request parser implementation that is fully compliant to the wire protocol
#[derive(Debug, Copy, Clone)]
pub struct CompliantParser;

impl CompliantParser {
    /// Parse the remainder of a request that started with 'PX '
    #[inline(always)]
    fn parse_req_px_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Request, ParseErr> {
        let x = words.next().ok_or(ParseErr::InvalidCommand)?;
        let y = words.next().ok_or(ParseErr::InvalidCommand)?;

        let x = usize::from_ascii(x).map_err(|_| ParseErr::InvalidCommand)?;
        let y = usize::from_ascii(y).map_err(|_| ParseErr::InvalidCommand)?;

        match words.next() {
            None => {
                std::hint::cold_path();
                Ok(Request::GetPixel { x, y })
            }
            Some(color) => {
                // TODO: Actually implement
                let color = u32::from_ascii_radix(color, 16).map_err(|_| ParseErr::InvalidCommand)?;
                let color = Color::from(color);
                Ok(Request::SetPixel { x, y, color })
            }
        }
    }

    #[inline(always)]
    fn parse_resp_px_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Response, ParseErr> {
        let x = words.next().ok_or(ParseErr::InvalidCommand)?;
        let x = usize::from_ascii(x).map_err(|_| ParseErr::InvalidCommand)?;

        let y = words.next().ok_or(ParseErr::InvalidCommand)?;
        let y = usize::from_ascii(y).map_err(|_| ParseErr::InvalidCommand)?;

        let color = words.next().ok_or(ParseErr::InvalidCommand)?;
        let color = u32::from_ascii_radix(color, 16)
            .map_err(|_| ParseErr::InvalidCommand)?
            .into();

        Ok(Response::PxData { x, y, color })
    }

    /// Parse the remainder of a request that sterted with 'HELP '
    #[inline(always)]
    fn parse_req_help_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Request, ParseErr> {
        match words.next() {
            None => Ok(Request::Help(HelpTopic::General)),
            Some(b"GENERAL" | b"general") => Ok(Request::Help(HelpTopic::General)),
            Some(b"SIZE" | b"size") => Ok(Request::Help(HelpTopic::Size)),
            Some(b"PX" | b"px") => Ok(Request::Help(HelpTopic::Px)),
            _ => Ok(Request::Help(HelpTopic::General)),
        }
    }

    #[inline(always)]
    fn parse_resp_help_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Response, ParseErr> {
        let topic = words.next().ok_or(ParseErr::InvalidCommand)?;
        match topic {
            b"HELP" => Ok(Response::Help(HelpTopic::General)),
            b"SIZE" => Ok(Response::Help(HelpTopic::Size)),
            b"PX" => Ok(Response::Help(HelpTopic::Px)),
            _ => Err(ParseErr::InvalidCommand),
        }
    }

    /// Parse the remainder of a request that started with 'SIZE '
    #[inline(always)]
    fn parse_req_size_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Request, ParseErr> {
        match words.next() {
            None => Ok(Request::GetSize),
            Some(_) => Err(ParseErr::InvalidCommand),
        }
    }

    #[inline(always)]
    fn parse_resp_size_prefixed<'a>(
        &self,
        mut words: impl Iterator<Item = &'a [u8]>,
    ) -> Result<Response, ParseErr> {
        let width = words.next().ok_or(ParseErr::InvalidCommand)?;
        let width = usize::from_ascii(width).map_err(|_| ParseErr::InvalidCommand)?;

        let height = words.next().ok_or(ParseErr::InvalidCommand)?;
        let height = usize::from_ascii(height).map_err(|_| ParseErr::InvalidCommand)?;

        Ok(Response::Size { width, height })
    }
}

impl RequestParser for CompliantParser {
    fn parse_request_bin(&self, line: &[u8]) -> Result<Request, ParseErr> {
        let mut words = line.split(|i| i == &b' ');

        match words.next().ok_or(ParseErr::UnknownCommand)? {
            b"PX" => self.parse_req_px_prefixed(words),
            b"HELP" => {
                std::hint::cold_path();
                self.parse_req_help_prefixed(words)
            }
            b"SIZE" => {
                std::hint::cold_path();
                self.parse_req_size_prefixed(words)
            }
            _ => Err(ParseErr::UnknownCommand),
        }
    }
}

impl ResponseParser for CompliantParser {
    fn parse_response_bin(&self, line: &[u8]) -> Result<Response, ParseErr> {
        let mut words = line.split(|i| i == &b' ');

        match words.next().ok_or(ParseErr::UnknownCommand)? {
            b"PX" => self.parse_resp_px_prefixed(words),
            b"HELP" => {
                std::hint::cold_path();
                self.parse_resp_help_prefixed(words)
            }
            b"SIZE" => {
                std::hint::cold_path();
                self.parse_resp_size_prefixed(words)
            }
            _ => Err(ParseErr::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod test {
    use ::test::Bencher;

    use super::super::test::*;
    use super::*;

    #[test]
    fn test_parse_commands() {
        let parser = CompliantParser;
        gen_test_parse_commands(parser);
    }

    #[bench]
    fn bench_parse_get_pixel(b: &mut Bencher) {
        let parser = CompliantParser;
        gen_bench_parse_get_pixel(parser, b);
    }

    #[bench]
    fn bench_parse_set_pixel(b: &mut Bencher) {
        let parser = CompliantParser;
        gen_bench_parse_set_pixel(parser, b);
    }

    #[bench]
    fn bench_parse_size(b: &mut Bencher) {
        let parser = CompliantParser;
        gen_bench_parse_size(parser, b);
    }
}
