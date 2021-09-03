#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Cpu { wants: u16, max: u16 },
    Powergrid { wants: u16, max: u16 },
    StructureZero,

    TooManyPassiveModules { wants: usize, max: u8 },
    TooManyTargetedModules { wants: usize, max: u8 },
    TooManyUntargetedModules { wants: usize, max: u8 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Cpu { wants, max } => write!(
                f,
                "Not enough CPU: Wants {} but only {} are available.",
                wants, max
            ),
            Error::Powergrid { wants, max } => write!(
                f,
                "Not enough Powergrid: Wants {} but only {} are available.",
                wants, max
            ),
            Error::StructureZero => {
                write!(f, "Not enough structure. It will explode while undocking.")
            }
            Error::TooManyPassiveModules { wants, max } => write!(
                f,
                "Too many passive modules: Wants {} but only {} are possible.",
                wants, max
            ),
            Error::TooManyTargetedModules { wants, max } => write!(
                f,
                "Too many targeted modules: Wants {} but only {} are possible.",
                wants, max
            ),
            Error::TooManyUntargetedModules { wants, max } => write!(
                f,
                "Too many untargeted modules: Wants {} but only {} are possible.",
                wants, max
            ),
        }
    }
}

impl std::error::Error for Error {}
