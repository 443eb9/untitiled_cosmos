use std::fmt::{Display, Formatter, Result};

pub enum PlanetGenerationError {
    MaxSystemRadiusExceeded,
}

impl Display for PlanetGenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PlanetGenerationError::MaxSystemRadiusExceeded => {
                write!(f, "Max system radius exceeded")
            }
        }
    }
}

pub enum MoonGenerationError {
    MinAccNotMet,
}

impl Display for MoonGenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MoonGenerationError::MinAccNotMet => {
                write!(f, "Minimum acceleration not met")
            }
        }
    }
}
