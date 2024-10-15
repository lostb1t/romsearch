use logos::Logos;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
   pub id: u64,
   pub name: String,
   pub location: String,
   pub size: Option<String>,
   pub date: Option<String>,
   pub platform: Option<PlatformKind>,
   pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone)]
pub enum PlatformKind {
    AMIGA,
    A26,
    A52,
    A78,
    GB,
    GBA,
    GBC,
    NES,
    SNES,
    WII,
    N64,
    DOS,
}

impl PlatformKind {
    pub fn from_name(input: String) -> Option<PlatformKind> {
        let s = &input.clone().to_lowercase();
        let mut lex = Token::lexer(&s);
        //let mut result = Platform::new(v.clone());
        let mut platform: Option<PlatformKind> = None;

        while let Some(token) = lex.next() {
            match token {
                // Token::VideoCodec(value) => {
                //     result.video_codec = Some(value);
                // }
                // Token::VideoResolution(value) => {
                //     result.video_resolution = Some(value);
                // }
                // Token::VideoSource(value) => {
                //     result.source = Some(value);
                // }
                // Token::AudioCodec(value) => {
                //     result.audio_codec = Some(value);
                // }
                // Token::AudioCodec(value) => {
                //     result.audio_codec = Some(value);
                // }
                // Token::Year(value) => {
                //     result.year = Some(value);
                // }
                Ok(Token::Platform(value)) => {
                    platform = Some(value);
                }
                _ => (),
            }
        }
        platform
    }
}

#[derive(Logos, Debug, PartialEq, Clone, Display)]
#[logos(subpattern year = r"(1[89]|20)\d\d")]
enum Token {

    // Platform
    #[regex(r" atari 2600 ", |_| PlatformKind::A26)]
    #[regex(r" atari 5200 ", |_| PlatformKind::A52)]
    #[regex(r" atari 7800 ", |_| PlatformKind::A78)]
    #[regex(r" amiga ", |_| PlatformKind::AMIGA)]
    #[regex(r" gba | game boy advance ", |_| PlatformKind::GBA)]
    #[regex(r" gbc | gameboy color | game boy color ", |_| PlatformKind::GBC)]
    #[regex(r" gb | gameboy | game boy ", |_| PlatformKind::GB)]
    #[regex(r" nintendo entertainment system ", |_| PlatformKind::NES)]
    #[regex(r" super nintendo entertainment system ", |_| PlatformKind::SNES)]
    #[regex(r" dos | ibm - pc and compatibles ", |_| PlatformKind::DOS)]
    #[regex(r" nintendo - Wii ", |_| PlatformKind::WII)]
    #[regex(r" nintendo 64 ", |_| PlatformKind::N64)]
    Platform(PlatformKind),
}
