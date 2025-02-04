//! NUT-06: Mint Information
//!
//! <https://github.com/cashubtc/nuts/blob/main/06.md>

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::{nut05, Error};

use super::nut01::PublicKey;
use super::nut04;

/// Mint Version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MintVersion {
    /// Mint Software name
    pub name: String,
    /// Mint Version
    pub version: String,
}

impl MintVersion {
    /// Create new [`MintVersion`]
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}

impl Serialize for MintVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let combined = format!("{}/{}", self.name, self.version);
        serializer.serialize_str(&combined)
    }
}

impl<'de> Deserialize<'de> for MintVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let combined = String::deserialize(deserializer)?;
        let parts: Vec<&str> = combined.split('/').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("Invalid input string"));
        }
        Ok(MintVersion {
            name: parts[0].to_string(),
            version: parts[1].to_string(),
        })
    }
}

/// Mint Info [NIP-06]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MintInfo<M, U> {
    /// name of the mint and should be recognizable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// hex pubkey of the mint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkey: Option<PublicKey>,
    /// implementation name and the version running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<MintVersion>,
    /// short description of the mint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// long description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_long: Option<String>,
    /// Contact info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<ContactInfo>>,
    /// shows which NUTs the mint supports
    pub nuts: NutsSettings<M, U>,
    /// Mint's icon URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Mint's endpoint URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    /// message of the day that the wallet must display to the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    /// server unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

impl<M, U> MintInfo<M, U> {
    /// Set name
    pub fn name<S>(self, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: Some(name.into()),
            ..self
        }
    }

    /// Set pubkey
    pub fn pubkey(self, pubkey: PublicKey) -> Self {
        Self {
            pubkey: Some(pubkey),
            ..self
        }
    }

    /// Set [`MintVersion`]
    pub fn version(self, mint_version: MintVersion) -> Self {
        Self {
            version: Some(mint_version),
            ..self
        }
    }

    /// Set description
    pub fn description<S>(self, description: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    /// Set long description
    pub fn long_description<S>(self, description_long: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            description_long: Some(description_long.into()),
            ..self
        }
    }

    /// Set contact info
    pub fn contact_info(self, contact_info: Vec<ContactInfo>) -> Self {
        Self {
            contact: Some(contact_info),
            ..self
        }
    }

    /// Set nuts
    pub fn nuts(self, nuts: NutsSettings<M, U>) -> Self {
        Self { nuts, ..self }
    }

    /// Set mint icon url
    pub fn icon_url<S>(self, icon_url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            icon_url: Some(icon_url.into()),
            ..self
        }
    }

    /// Set motd
    pub fn motd<S>(self, motd: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            motd: Some(motd.into()),
            ..self
        }
    }

    /// Set time
    pub fn time<S>(self, time: S) -> Self
    where
        S: Into<u64>,
    {
        Self {
            time: Some(time.into()),
            ..self
        }
    }
}

/// Supported nuts and settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutsSettings<M, U> {
    /// NUT04 Settings
    #[serde(rename = "4")]
    pub nut04: nut04::Settings<M, U>,
    // NUT05 Settings
    #[serde(rename = "5")]
    pub nut05: nut05::Settings<M, U>,
}

#[derive(Debug, Clone)]
pub struct NutsSettingsBuilder<M, U> {
    pub nut04: Option<nut04::Settings<M, U>>,
    pub nut05: Option<nut05::Settings<M, U>>,
}

impl<M, U> Default for NutsSettingsBuilder<M, U> {
    fn default() -> Self {
        Self {
            nut04: None,
            nut05: None,
        }
    }
}

impl<M, U> NutsSettingsBuilder<M, U> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nut_04(mut self, nut04_settings: nut04::Settings<M, U>) -> Self {
        self.nut04 = Some(nut04_settings);
        self
    }

    pub fn build(self) -> Result<NutsSettings<M, U>, NutsBuilderError> {
        let nut04 = self.nut04.ok_or(NutsBuilderError::MissingConfig(4))?;
        let nut05 = self.nut05.ok_or(NutsBuilderError::MissingConfig(5))?;

        Ok(NutsSettings { nut04, nut05 })
    }
}

#[derive(Debug, Error)]
pub enum NutsBuilderError {
    #[error("Config for nut{0} has not been set")]
    MissingConfig(u8),
}

/// Check state Settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub struct SupportedSettings {
    supported: bool,
}

/// Contact Info
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Contact Method i.e. nostr
    pub method: String,
    /// Contact info i.e. npub...
    pub info: String,
}

impl ContactInfo {
    /// Create new [`ContactInfo`]
    pub fn new(method: String, info: String) -> Self {
        Self { method, info }
    }
}

#[cfg(test)]
mod tests {

    use crate::traits::test_types::{TestMethod, TestUnit};

    use super::*;

    #[test]
    fn test_des_mint_into() {
        let mint_info_str = r#"{
    "name": "Cashu mint",
    "pubkey": "0296d0aa13b6a31cf0cd974249f28c7b7176d7274712c95a41c7d8066d3f29d679",
    "version": "Nutshell/0.15.3",
    "contact": [
        ["", ""],
        ["", ""]
    ],
    "nuts": {
        "4": {
            "methods": [
                {"method": "bolt11", "unit": "sat", "description": true},
                {"method": "bolt11", "unit": "usd", "description": true}
            ],
            "disabled": false
        },
        "5": {
            "methods": [
                {"method": "bolt11", "unit": "sat"},
                {"method": "bolt11", "unit": "usd"}
            ],
            "disabled": false
        },
        "7": {"supported": true},
        "8": {"supported": true},
        "9": {"supported": true},
        "10": {"supported": true},
        "11": {"supported": true}
    }
}"#;

        let _mint_info: MintInfo<TestMethod, TestUnit> =
            serde_json::from_str(mint_info_str).unwrap();
    }

    #[test]
    fn test_ser_mint_info() {
        /*
                let mint_info = serde_json::to_string(&MintInfo {
                    name: Some("Cashu-crab".to_string()),
                    pubkey: None,
                    version: None,
                    description: Some("A mint".to_string()),
                    description_long: Some("Some longer test".to_string()),
                    contact: None,
                    nuts: Nuts::default(),
                    motd: None,
                })
                .unwrap();

                println!("{}", mint_info);
        */
        let mint_info_str = r#"{
  "name": "Bob's Cashu mint",
  "pubkey": "0283bf290884eed3a7ca2663fc0260de2e2064d6b355ea13f98dec004b7a7ead99",
  "version": "Nutshell/0.15.0",
  "description": "The short mint description",
  "description_long": "A description that can be a long piece of text.",
  "contact": [
    {
        "method": "nostr",
        "info": "xxxxx"
    },
    {
        "method": "email",
        "info": "contact@me.com"
    }
  ],
  "motd": "Message to display to users.",
  "icon_url": "https://this-is-a-mint-icon-url.com/icon.png",
  "nuts": {
    "4": {
      "methods": [
        {
        "method": "bolt11",
        "unit": "sat",
        "min_amount": 0,
        "max_amount": 10000,
        "description": true
        }
      ],
      "disabled": false
    },
    "5": {
      "methods": [
        {
        "method": "bolt11",
        "unit": "sat",
        "min_amount": 0,
        "max_amount": 10000
        }
      ],
      "disabled": false
    },
    "7": {"supported": true},
    "8": {"supported": true},
    "9": {"supported": true},
    "10": {"supported": true},
    "12": {"supported": true}
  }
}"#;
        let info: MintInfo<TestMethod, TestUnit> = serde_json::from_str(mint_info_str).unwrap();
        let mint_info_str = r#"{
  "name": "Bob's Cashu mint",
  "pubkey": "0283bf290884eed3a7ca2663fc0260de2e2064d6b355ea13f98dec004b7a7ead99",
  "version": "Nutshell/0.15.0",
  "description": "The short mint description",
  "description_long": "A description that can be a long piece of text.",
  "contact": [
        ["nostr", "xxxxx"],
        ["email", "contact@me.com"]
  ],
  "motd": "Message to display to users.",
  "icon_url": "https://this-is-a-mint-icon-url.com/icon.png",
  "nuts": {
    "4": {
      "methods": [
        {
        "method": "bolt11",
        "unit": "sat",
        "min_amount": 0,
        "max_amount": 10000,
        "description": true
        }
      ],
      "disabled": false
    },
    "5": {
      "methods": [
        {
        "method": "bolt11",
        "unit": "sat",
        "min_amount": 0,
        "max_amount": 10000
        }
      ],
      "disabled": false
    },
    "7": {"supported": true},
    "8": {"supported": true},
    "9": {"supported": true},
    "10": {"supported": true},
    "12": {"supported": true}
  }
}"#;
        let mint_info: MintInfo<TestMethod, TestUnit> =
            serde_json::from_str(mint_info_str).unwrap();

        assert_eq!(info, mint_info);
    }
}
