use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use strum_macros::Display;

fn platforms() -> &'static HashMap<PlatformKind, Platform> {
    static HASHMAP: OnceLock<HashMap<PlatformKind, Platform>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        HashMap::from([
            (
                PlatformKind::GB,
                Platform {
                    weight: 100,
                    tags: Some(vec![
                        "game boy".to_string(),
                        "gameboy".to_string(),
                        "nintendo".to_string(),
                    ]),
                    kind: PlatformKind::GB,
                    regex: Some(r"nintendo - game boy".to_string()),
                },
            ),
            (
                PlatformKind::GBA,
                Platform {
                    weight: 99,
                    tags: vec![
                        "game boy advanced".to_string(),
                        "gameboy advanced".to_string(),
                        "nintendo".to_string(),
                    ].into(),
                    kind: PlatformKind::GBA,
                    regex: Some(r" gba |game boy advance".to_string()),
                },
            ),
            (
                PlatformKind::GBC,
                Platform {
                    weight: 98,
                    tags: vec![
                        "game boy color".to_string(),
                        "gameboy color".to_string(),
                        "nintendo".to_string(),
                    ].into(),
                    kind: PlatformKind::GBC,
                    regex: Some(r" gbc |gameboy color|game boy color".to_string()),
                },
            ),
            (
                PlatformKind::NES,
                Platform {
                    weight: 97,
                    tags: vec![
                        "nintendo entertainment system".to_string(),
                        "nintendo".to_string(),
                    ].into(),
                    kind: PlatformKind::NES,
                    regex: Some(r"nintendo entertainment system".to_string()),
                },
            ),
            (
                PlatformKind::SNES,
                Platform {
                    weight: 96,
                    tags: vec![
                        "super nintendo entertainment system".to_string(),
                        "nintendo".to_string(),
                    ].into(),
                    kind: PlatformKind::SNES,
                    regex: Some(r"super nintendo entertainment system".to_string()),
                },
            ),
            (
                PlatformKind::WII,
                Platform {
                    weight: 95,
                    tags: vec!["nintendo".to_string()].into(),
                    kind: PlatformKind::WII,
                    regex: Some(r"nintendo - wii".to_string()),
                },
            ),
            (
                PlatformKind::WIIU,
                Platform {
                    weight: 0,
                    tags: vec!["nintendo".to_string()].into(),
                    kind: PlatformKind::WIIU,
                    regex: Some(r"nintendo - wii u".to_string()),
                },
            ),
            (
                PlatformKind::N64,
                Platform {
                    weight: 94,
                    tags: vec!["nintendo".to_string()].into(),
                    kind: PlatformKind::N64,
                    regex: Some(r"nintendo n64".to_string()),
                },
            ),
            (
                PlatformKind::DS,
                Platform {
                    weight: 93,
                    tags: vec!["nintendo".to_string()].into(),
                    kind: PlatformKind::DS,
                    regex: Some(r"nintendo ds".to_string()),
                },
            ),
            (
                PlatformKind::N3DS,
                Platform {
                    weight: 92,
                    tags: vec!["nintendo".to_string()].into(),
                    kind: PlatformKind::N3DS,
                    regex: Some(r"nintendo 3ds | 3ds ".to_string()),
                },
            ),
            (
                PlatformKind::GC,
                Platform {
                    weight: 91,
                    tags: vec!["nintendo".to_string(), "gamecube".to_string()].into(),
                    kind: PlatformKind::GC,
                    regex: Some(r"nintendo gamecube".to_string()),
                },
            ),
            (
                PlatformKind::DOS,
                Platform {
                    weight: 90,
                    tags: vec!["msdos".to_string(), "pc".to_string()].into(),
                    kind: PlatformKind::DOS,
                    regex: Some(r" dos |ibm - pc compatible|ibm - pc and compatibles".to_string()),
                },
            ),
            (
                PlatformKind::A26,
                Platform {
                    weight: 0,
                    tags: vec!["2600".to_string()].into(),
                    kind: PlatformKind::A26,
                    regex: Some(r"atari 2600".to_string()),
                },
            ),
            (
                PlatformKind::A52,
                Platform {
                    weight: 0,
                    tags: vec!["5200".to_string()].into(),
                    kind: PlatformKind::A52,
                    regex: Some(r"atari 5200".to_string()),
                },
            ),
            (
                PlatformKind::A78,
                Platform {
                    weight: 0,
                    tags: vec!["7800".to_string()].into(),
                    kind: PlatformKind::A78,
                    regex: Some(r"atari 7800".to_string()),
                },
            ),
            (
                PlatformKind::AMIGA,
                Platform {
                    weight: 0,
                    tags: vec!["commodore".to_string()].into(),
                    kind: PlatformKind::AMIGA,
                    regex: Some(r"commodore - amiga".to_string()),
                },
            ),
            (
                PlatformKind::ARCADE,
                Platform {
                    weight: 0,
                    tags: vec![].into(),
                    kind: PlatformKind::ARCADE,
                    regex: Some(r"arcade".to_string()),
                },
            ),
            (
                PlatformKind::PS1,
                Platform {
                    weight: 0,
                    tags: vec!["sony".to_string()].into(),
                    kind: PlatformKind::PS1,
                    regex: Some(r"\bplaystation\b(?!\s*\d)".to_string()),
                },
            ),
            (
                PlatformKind::PS2,
                Platform {
                    weight: 0,
                    tags: vec!["sony".to_string()].into(),
                    kind: PlatformKind::PS2,
                    regex: Some(r"playstation 2".to_string()),
                },
            ),
            (
                PlatformKind::PS3,
                Platform {
                    weight: 0,
                    tags: vec!["sony".to_string()].into(),
                    kind: PlatformKind::PS3,
                    regex: Some(r"playstation 3".to_string()),
                },
            ),
            (
                PlatformKind::PSP,
                Platform {
                    weight: 0,
                    tags: vec!["sony".to_string()].into(),
                    kind: PlatformKind::PSP,
                    regex: Some(r"playstation portable".to_string()),
                },
            ),
        ])
    })
}

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub struct Platform {
    pub weight: usize,
    pub kind: PlatformKind,
    pub tags: Option<Vec<String>>,
    pub regex: Option<String>,
}

impl Platform {
    pub fn for_kind(kind: &PlatformKind) -> Option<&Platform> {
        platforms().get(kind)
    }

    pub fn parse(input: &String) -> Option<&Platform> {
        for platform in platforms() {
            if platform.1.regex.is_some() {
                let re = match Regex::new(&platform.1.regex.clone().unwrap()) {
                    Ok(re) => re,
                    Err(err) => {
                        dbg!("Error compiling regex: {}", err);
                        return None;
                    }
                };
                if re.is_match(input.to_lowercase().as_str()) {
                    return Some(platform.1);
                }
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Display, Clone, Hash)]
pub enum PlatformKind {
    AMIGA,
    ARCADE,
    A26,
    A52,
    A78,
    GB,
    GBA,
    GBC,
    NES,
    PS1,
    PS2,
    PS3,
    PSP,
    SNES,
    WII,
    WIIU,
    N64,
    DS,
    #[strum(to_string = "3DS")]
    N3DS,
    GC,
    DOS,
}
