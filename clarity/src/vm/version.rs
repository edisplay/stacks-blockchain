use crate::vm::errors::{Error, RuntimeErrorType};
use stacks_common::types::StacksEpochId;
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ClarityVersion {
    Clarity1,
    Clarity2,
}

impl fmt::Display for ClarityVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClarityVersion::Clarity1 => write!(f, "Clarity 1"),
            ClarityVersion::Clarity2 => write!(f, "Clarity 2"),
        }
    }
}

impl ClarityVersion {
    pub fn latest() -> ClarityVersion {
        ClarityVersion::Clarity2
    }
    pub fn default_for_epoch(epoch_id: StacksEpochId) -> ClarityVersion {
        match epoch_id {
            StacksEpochId::Epoch10 => {
                warn!("Attempted to get default Clarity version for Epoch 1.0 where Clarity does not exist");
                ClarityVersion::Clarity1
            }
            StacksEpochId::Epoch20 => ClarityVersion::Clarity1,
            StacksEpochId::Epoch2_05 => ClarityVersion::Clarity1,
            StacksEpochId::Epoch21 => ClarityVersion::Clarity2,
            StacksEpochId::Epoch22 => ClarityVersion::Clarity2,
            StacksEpochId::Epoch23 => ClarityVersion::Clarity2,
            StacksEpochId::Epoch24 => ClarityVersion::Clarity2,
        }
    }
}

impl FromStr for ClarityVersion {
    type Err = Error;

    fn from_str(version: &str) -> Result<ClarityVersion, Error> {
        let s = version.to_string().to_lowercase();
        if s == "clarity1" {
            Ok(ClarityVersion::Clarity1)
        } else if s == "clarity2" {
            Ok(ClarityVersion::Clarity2)
        } else {
            Err(RuntimeErrorType::ParseError(
                "Invalid clarity version. Valid versions are: Clarity1, Clarity2.".to_string(),
            )
            .into())
        }
    }
}
