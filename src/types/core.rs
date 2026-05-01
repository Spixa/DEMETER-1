use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub fn hash_bytes(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

pub fn hash_parameter<T: Serialize>(param: &T) -> [u8; 32] {
    let bytes = bincode::serialize(param).unwrap();
    hash_bytes(&bytes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NationId(pub u16);

impl NationId {
    pub const JAPAN: Self = Self(1);
    pub const USA: Self = Self(2);
    pub const CHILE: Self = Self(3);
    pub const INDONESIA: Self = Self(4);
    pub const GERMANY: Self = Self(5);
    pub const SOUTH_KOREA: Self = Self(6);
    pub const NEW_ZEALAND: Self = Self(7);

    pub const ALL: [Self; 7] = [
        Self::JAPAN,
        Self::USA,
        Self::CHILE,
        Self::INDONESIA,
        Self::GERMANY,
        Self::SOUTH_KOREA,
        Self::NEW_ZEALAND,
    ];

    pub fn name(&self) -> &'static str {
        match self.0 {
            1 => "Japan",
            2 => "USA",
            3 => "Chile",
            4 => "Indonesia",
            5 => "Germany",
            6 => "South Korea",
            7 => "New Zealand",
            _ => "Unknown",
        }
    }

    pub fn is_veto_power(&self, zone: Zone) -> bool {
        // the following code looks ugly af but i need this to be a tuple for later use
        match zone {
            Zone::MagmaConduit => self.0 == Self::JAPAN.0 || self.0 == Self::USA.0,
            Zone::HeatExchanger => self.0 == Self::GERMANY.0,
            Zone::NeutrinoObservatory => self.0 == Self::SOUTH_KOREA.0,
            Zone::EmergencySystems => self.0 == Self::CHILE.0, // chilean lives hang in the balance
        }
    }
}

// README contains more information about the zones and their parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zone {
    MagmaConduit,
    HeatExchanger,
    NeutrinoObservatory,
    EmergencySystems,
}

impl Zone {
    pub fn required_quorum(&self) -> usize {
        match self {
            Zone::MagmaConduit | Zone::HeatExchanger | Zone::NeutrinoObservatory => 5,
            Zone::EmergencySystems => 6, // more stakes or something
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Zone::MagmaConduit => "Magma Conduit",
            Zone::HeatExchanger => "Heat Exchanger Farm",
            Zone::NeutrinoObservatory => "Neutrino Observatory", // i saw this from a NASA documentary
            Zone::EmergencySystems => "Emergency Systems",
        }
    }
}
