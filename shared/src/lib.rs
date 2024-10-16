use logos::Logos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub size: Option<String>,
    pub date: Option<String>,
    pub platform: Option<Platform>,
    // pub tags: Vec<String>,
    // pub weight: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Platform {
    pub weight: usize,
    pub kind: PlatformKind,
    pub tags: Vec<String>,
}

impl Platform {
    // fn platforms() -> HashMap<PlatformKind, Platform> {
    //   HashMap::from([
    //     (PlatformKind::AMIGA, Platform {
    //       weight: 0,
    //       kind: PlatformKind::AMIGA
    //     }),
    //     (PlatformKind::GB, Platform {
    //       weight: 20,
    //       kind: PlatformKind::GB
    //     })
    //   ]
    // }

    pub fn for_kind(kind: &PlatformKind) -> Option<Platform> {
        match kind {
            PlatformKind::GB => Some(Platform {
                weight: 100,
                tags: vec![
                    "game boy".to_string(),
                    "gameboy".to_string(),
                    "nintendo".to_string(),
                ],
                kind: PlatformKind::GB,
            }),
            PlatformKind::GBA => Some(Platform {
                weight: 99,
                tags: vec![
                    "game boy advanced".to_string(),
                    "gameboy advanced".to_string(),
                    "nintendo".to_string(),
                ],
                kind: PlatformKind::GBA,
            }),
            PlatformKind::GBC => Some(Platform {
                weight: 98,
                tags: vec![
                    "game boy color".to_string(),
                    "gameboy color".to_string(),
                    "nintendo".to_string(),
                ],
                kind: PlatformKind::GBC,
            }),
            PlatformKind::NES => Some(Platform {
                weight: 97,
                tags: vec![
                    "nintendo entertainment system".to_string(),
                    "nintendo".to_string(),
                ],
                kind: PlatformKind::NES,
            }),
            PlatformKind::SNES => Some(Platform {
                weight: 96,
                tags: vec![
                    "super nintendo entertainment system".to_string(),
                    "nintendo".to_string(),
                ],
                kind: PlatformKind::SNES,
            }),
            PlatformKind::WII => Some(Platform {
                weight: 95,
                tags: vec!["nintendo".to_string(), "superduperteststring".to_string()"],
                kind: PlatformKind::WII,
            }),
            _ => None,
        }
        // platforms.into_iter().find(|x|x.kind == kind)
    }
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
    #[regex(r"atari 2600", |_| PlatformKind::A26)]
    #[regex(r"atari 5200", |_| PlatformKind::A52)]
    #[regex(r"atari 7800", |_| PlatformKind::A78)]
    #[regex(r"amiga", |_| PlatformKind::AMIGA)]
    #[regex(r" gba |game boy advance", |_| PlatformKind::GBA)]
    #[regex(r" gbc |gameboy color|game boy color", |_| PlatformKind::GBC)]
    #[regex(r"nintendo - game boy", |_| PlatformKind::GB)]
    #[regex(r"nintendo entertainment system", |_| PlatformKind::NES)]
    #[regex(r"super nintendo entertainment system", |_| PlatformKind::SNES)]
    #[regex(r" dos |ibm - pc and compatibles", |_| PlatformKind::DOS)]
    #[regex(r"nintendo - wii", |_| PlatformKind::WII)]
    #[regex(r"nintendo 64", |_| PlatformKind::N64)]
    Platform(PlatformKind),
}
