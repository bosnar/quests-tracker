use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

// เขียน Impl เพื่อใช้ .to_string() ได้ใน enum stage
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };

        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Self::Local),
            "Development" => Ok(Self::Development),
            "Production" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid stage: {}", stage)),
        }
    }
}
