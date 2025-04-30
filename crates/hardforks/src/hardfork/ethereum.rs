use crate::{ethereum::mainnet::*, hardfork, ForkCondition};
use alloc::vec::Vec;
use alloy_chains::Chain;
use alloy_primitives::{uint, U256};

hardfork!(
    /// The name of an Ethereum hardfork.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    EthereumHardfork {
        /// Frontier: <https://blog.ethereum.org/2015/03/03/ethereum-launch-process>.
        Frontier,
        /// Homestead: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/homestead.md>.
        Homestead,
        /// The DAO fork: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/dao-fork.md>.
        Dao,
        /// Tangerine: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/tangerine-whistle.md>.
        Tangerine,
        /// Spurious Dragon: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/spurious-dragon.md>.
        SpuriousDragon,
        /// Byzantium: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/byzantium.md>.
        Byzantium,
        /// Constantinople: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/constantinople.md>.
        Constantinople,
        /// Petersburg: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/petersburg.md>.
        Petersburg,
        /// Istanbul: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/istanbul.md>.
        Istanbul,
        /// Muir Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/muir-glacier.md>.
        MuirGlacier,
        /// Berlin: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/berlin.md>.
        Berlin,
        /// London: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/london.md>.
        London,
        /// Arrow Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/arrow-glacier.md>.
        ArrowGlacier,
        /// Gray Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/gray-glacier.md>.
        GrayGlacier,
        /// Paris: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/paris.md>.
        Paris,
        /// Shanghai: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/shanghai.md>.
        Shanghai,
        /// Cancun: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/cancun.md>
        Cancun,
        /// Prague.
        Prague,
        /// Osaka: <https://eips.ethereum.org/EIPS/eip-7607>
        Osaka,
    }
);

impl EthereumHardfork {
    /// Retrieves the activation block for the specified hardfork on the given chain.
    pub fn activation_block(&self, chain: Chain) -> Option<u64> {
        if chain == Chain::mainnet() {
            return self.mainnet_activation_block();
        }
        if chain == Chain::sepolia() {
            return self.sepolia_activation_block();
        }
        if chain == Chain::holesky() {
            return self.holesky_activation_block();
        }
        if chain == Chain::hoodi() {
            return self.hoodi_activation_block();
        }

        None
    }

    /// Retrieves the activation block for the specified hardfork on the Ethereum mainnet.
    pub const fn mainnet_activation_block(&self) -> Option<u64> {
        match self {
            Self::Frontier => Some(MAINNET_FRONTIER_BLOCK),
            Self::Homestead => Some(MAINNET_HOMESTEAD_BLOCK),
            Self::Dao => Some(MAINNET_DAO_BLOCK),
            Self::Tangerine => Some(MAINNET_TANGERINE_BLOCK),
            Self::SpuriousDragon => Some(MAINNET_SPURIOUS_DRAGON_BLOCK),
            Self::Byzantium => Some(MAINNET_BYZANTIUM_BLOCK),
            Self::Constantinople => Some(MAINNET_CONSTANTINOPLE_BLOCK),
            Self::Petersburg => Some(MAINNET_PETERSBURG_BLOCK),
            Self::Istanbul => Some(MAINNET_ISTANBUL_BLOCK),
            Self::MuirGlacier => Some(MAINNET_MUIR_GLACIER_BLOCK),
            Self::Berlin => Some(MAINNET_BERLIN_BLOCK),
            Self::London => Some(MAINNET_LONDON_BLOCK),
            Self::ArrowGlacier => Some(MAINNET_ARROW_GLACIER_BLOCK),
            Self::GrayGlacier => Some(MAINNET_GRAY_GLACIER_BLOCK),
            Self::Paris => Some(MAINNET_PARIS_BLOCK),
            Self::Shanghai => Some(MAINNET_SHANGHAI_BLOCK),
            Self::Cancun => Some(MAINNET_CANCUN_BLOCK),
            _ => None,
        }
    }

    /// Retrieves the activation block for the specified hardfork on the Sepolia testnet.
    pub const fn sepolia_activation_block(&self) -> Option<u64> {
        match self {
            Self::Paris => Some(1450409),
            Self::Shanghai => Some(2990908),
            Self::Cancun => Some(5187023),
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier => Some(0),
            _ => None,
        }
    }

    /// Retrieves the activation block for the specified hardfork on the holesky testnet.
    const fn holesky_activation_block(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(0),
            Self::Shanghai => Some(6698),
            Self::Cancun => Some(894733),
            _ => None,
        }
    }

    /// Retrieves the activation block for the specified hardfork on the hoodi testnet.
    const fn hoodi_activation_block(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris
            | Self::Shanghai
            | Self::Cancun => Some(0),
            _ => None,
        }
    }

    /// Retrieves the activation block for the specified hardfork on the Arbitrum Sepolia testnet.
    pub const fn arbitrum_sepolia_activation_block(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(0),
            Self::Shanghai => Some(10653737),
            // Hardfork::ArbOS11 => Some(10653737),
            Self::Cancun => Some(18683405),
            // Hardfork::ArbOS20Atlas => Some(18683405),
            _ => None,
        }
    }

    /// Retrieves the activation block for the specified hardfork on the Arbitrum One mainnet.
    pub const fn arbitrum_activation_block(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(0),
            Self::Shanghai => Some(184097479),
            // Hardfork::ArbOS11 => Some(184097479),
            Self::Cancun => Some(190301729),
            // Hardfork::ArbOS20Atlas => Some(190301729),
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the given chain.
    pub fn activation_timestamp(&self, chain: Chain) -> Option<u64> {
        if chain == Chain::mainnet() {
            return self.mainnet_activation_timestamp();
        }
        if chain == Chain::sepolia() {
            return self.sepolia_activation_timestamp();
        }
        if chain == Chain::holesky() {
            return self.holesky_activation_timestamp();
        }
        if chain == Chain::hoodi() {
            return self.hoodi_activation_timestamp();
        }

        None
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Ethereum mainnet.
    pub const fn mainnet_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Frontier => Some(MAINNET_FRONTIER_TIMESTAMP),
            Self::Homestead => Some(MAINNET_HOMESTEAD_TIMESTAMP),
            Self::Dao => Some(MAINNET_DAO_TIMESTAMP),
            Self::Tangerine => Some(MAINNET_TANGERINE_TIMESTAMP),
            Self::SpuriousDragon => Some(MAINNET_SPURIOUS_DRAGON_TIMESTAMP),
            Self::Byzantium => Some(MAINNET_BYZANTIUM_TIMESTAMP),
            Self::Constantinople => Some(MAINNET_CONSTANTINOPLE_TIMESTAMP),
            Self::Petersburg => Some(MAINNET_PETERSBURG_TIMESTAMP),
            Self::Istanbul => Some(MAINNET_ISTANBUL_TIMESTAMP),
            Self::MuirGlacier => Some(MAINNET_MUIR_GLACIER_TIMESTAMP),
            Self::Berlin => Some(MAINNET_BERLIN_TIMESTAMP),
            Self::London => Some(MAINNET_LONDON_TIMESTAMP),
            Self::ArrowGlacier => Some(MAINNET_ARROW_GLACIER_TIMESTAMP),
            Self::GrayGlacier => Some(MAINNET_GRAY_GLACIER_TIMESTAMP),
            Self::Paris => Some(MAINNET_PARIS_TIMESTAMP),
            Self::Shanghai => Some(MAINNET_SHANGHAI_TIMESTAMP),
            Self::Cancun => Some(MAINNET_CANCUN_TIMESTAMP),
            Self::Prague => Some(MAINNET_PRAGUE_TIMESTAMP),
            // upcoming hardforks
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Sepolia testnet.
    pub const fn sepolia_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(1633267481),
            Self::Shanghai => Some(1677557088),
            Self::Cancun => Some(1706655072),
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Holesky testnet.
    pub const fn holesky_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(1695902100),
            Self::Shanghai => Some(1696000704),
            Self::Cancun => Some(1707305664),
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Hoodi testnet.
    pub const fn hoodi_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Prague => Some(1742999832),
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris
            | Self::Shanghai
            | Self::Cancun => Some(0),
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Arbitrum Sepolia
    /// testnet.
    pub const fn arbitrum_sepolia_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(1692726996),
            Self::Shanghai => Some(1706634000),
            // Hardfork::ArbOS11 => Some(1706634000),
            Self::Cancun => Some(1709229600),
            // Hardfork::ArbOS20Atlas => Some(1709229600),
            _ => None,
        }
    }

    /// Retrieves the activation timestamp for the specified hardfork on the Arbitrum One mainnet.
    pub const fn arbitrum_activation_timestamp(&self) -> Option<u64> {
        match self {
            Self::Frontier
            | Self::Homestead
            | Self::Dao
            | Self::Tangerine
            | Self::SpuriousDragon
            | Self::Byzantium
            | Self::Constantinople
            | Self::Petersburg
            | Self::Istanbul
            | Self::MuirGlacier
            | Self::Berlin
            | Self::London
            | Self::ArrowGlacier
            | Self::GrayGlacier
            | Self::Paris => Some(1622240000),
            Self::Shanghai => Some(1708804873),
            // Hardfork::ArbOS11 => Some(1708804873),
            Self::Cancun => Some(1710424089),
            // Hardfork::ArbOS20Atlas => Some(1710424089),
            _ => None,
        }
    }

    /// Ethereum mainnet list of hardforks.
    pub const fn mainnet() -> [(Self, ForkCondition); 18] {
        [
            (Self::Frontier, ForkCondition::Block(MAINNET_FRONTIER_BLOCK)),
            (Self::Homestead, ForkCondition::Block(MAINNET_HOMESTEAD_BLOCK)),
            (Self::Dao, ForkCondition::Block(MAINNET_DAO_BLOCK)),
            (Self::Tangerine, ForkCondition::Block(MAINNET_TANGERINE_BLOCK)),
            (Self::SpuriousDragon, ForkCondition::Block(MAINNET_SPURIOUS_DRAGON_BLOCK)),
            (Self::Byzantium, ForkCondition::Block(MAINNET_BYZANTIUM_BLOCK)),
            (Self::Constantinople, ForkCondition::Block(MAINNET_CONSTANTINOPLE_BLOCK)),
            (Self::Petersburg, ForkCondition::Block(MAINNET_PETERSBURG_BLOCK)),
            (Self::Istanbul, ForkCondition::Block(MAINNET_ISTANBUL_BLOCK)),
            (Self::MuirGlacier, ForkCondition::Block(MAINNET_MUIR_GLACIER_BLOCK)),
            (Self::Berlin, ForkCondition::Block(MAINNET_BERLIN_BLOCK)),
            (Self::London, ForkCondition::Block(MAINNET_LONDON_BLOCK)),
            (Self::ArrowGlacier, ForkCondition::Block(MAINNET_ARROW_GLACIER_BLOCK)),
            (Self::GrayGlacier, ForkCondition::Block(MAINNET_GRAY_GLACIER_BLOCK)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: MAINNET_PARIS_BLOCK,
                    fork_block: None,
                    total_difficulty: uint!(58_750_000_000_000_000_000_000_U256),
                },
            ),
            (Self::Shanghai, ForkCondition::Timestamp(MAINNET_SHANGHAI_TIMESTAMP)),
            (Self::Cancun, ForkCondition::Timestamp(MAINNET_CANCUN_TIMESTAMP)),
            (Self::Prague, ForkCondition::Timestamp(MAINNET_PRAGUE_TIMESTAMP)),
        ]
    }

    /// Ethereum sepolia list of hardforks.
    pub const fn sepolia() -> [(Self, ForkCondition); 16] {
        [
            (Self::Frontier, ForkCondition::Block(0)),
            (Self::Homestead, ForkCondition::Block(0)),
            (Self::Dao, ForkCondition::Block(0)),
            (Self::Tangerine, ForkCondition::Block(0)),
            (Self::SpuriousDragon, ForkCondition::Block(0)),
            (Self::Byzantium, ForkCondition::Block(0)),
            (Self::Constantinople, ForkCondition::Block(0)),
            (Self::Petersburg, ForkCondition::Block(0)),
            (Self::Istanbul, ForkCondition::Block(0)),
            (Self::MuirGlacier, ForkCondition::Block(0)),
            (Self::Berlin, ForkCondition::Block(0)),
            (Self::London, ForkCondition::Block(0)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: 1450409,
                    fork_block: Some(1735371),
                    total_difficulty: uint!(17_000_000_000_000_000_U256),
                },
            ),
            (Self::Shanghai, ForkCondition::Timestamp(1677557088)),
            (Self::Cancun, ForkCondition::Timestamp(1706655072)),
            (Self::Prague, ForkCondition::Timestamp(1741159776)),
        ]
    }

    /// Ethereum holesky list of hardforks.
    pub const fn holesky() -> [(Self, ForkCondition); 16] {
        [
            (Self::Frontier, ForkCondition::Block(0)),
            (Self::Homestead, ForkCondition::Block(0)),
            (Self::Dao, ForkCondition::Block(0)),
            (Self::Tangerine, ForkCondition::Block(0)),
            (Self::SpuriousDragon, ForkCondition::Block(0)),
            (Self::Byzantium, ForkCondition::Block(0)),
            (Self::Constantinople, ForkCondition::Block(0)),
            (Self::Petersburg, ForkCondition::Block(0)),
            (Self::Istanbul, ForkCondition::Block(0)),
            (Self::MuirGlacier, ForkCondition::Block(0)),
            (Self::Berlin, ForkCondition::Block(0)),
            (Self::London, ForkCondition::Block(0)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: 0,
                    fork_block: Some(0),
                    total_difficulty: U256::ZERO,
                },
            ),
            (Self::Shanghai, ForkCondition::Timestamp(1696000704)),
            (Self::Cancun, ForkCondition::Timestamp(1707305664)),
            (Self::Prague, ForkCondition::Timestamp(1740434112)),
        ]
    }

    /// Ethereum Hoodi list of hardforks.
    pub const fn hoodi() -> [(Self, ForkCondition); 16] {
        [
            (Self::Frontier, ForkCondition::Block(0)),
            (Self::Homestead, ForkCondition::Block(0)),
            (Self::Dao, ForkCondition::Block(0)),
            (Self::Tangerine, ForkCondition::Block(0)),
            (Self::SpuriousDragon, ForkCondition::Block(0)),
            (Self::Byzantium, ForkCondition::Block(0)),
            (Self::Constantinople, ForkCondition::Block(0)),
            (Self::Petersburg, ForkCondition::Block(0)),
            (Self::Istanbul, ForkCondition::Block(0)),
            (Self::MuirGlacier, ForkCondition::Block(0)),
            (Self::Berlin, ForkCondition::Block(0)),
            (Self::London, ForkCondition::Block(0)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: 0,
                    fork_block: Some(0),
                    total_difficulty: U256::ZERO,
                },
            ),
            (Self::Shanghai, ForkCondition::Timestamp(0)),
            (Self::Cancun, ForkCondition::Timestamp(0)),
            (Self::Prague, ForkCondition::Timestamp(1742999832)),
        ]
    }
}

/// Helper methods for Ethereum forks.
#[auto_impl::auto_impl(&, Arc)]
pub trait EthereumHardforks {
    /// Retrieves [`ForkCondition`] by an [`EthereumHardfork`]. If `fork` is not present, returns
    /// [`ForkCondition::Never`].
    fn ethereum_fork_activation(&self, fork: EthereumHardfork) -> ForkCondition;

    /// Convenience method to check if an [`EthereumHardfork`] is active at a given timestamp.
    fn is_ethereum_fork_active_at_timestamp(&self, fork: EthereumHardfork, timestamp: u64) -> bool {
        self.ethereum_fork_activation(fork).active_at_timestamp(timestamp)
    }

    /// Convenience method to check if an [`EthereumHardfork`] is active at a given block number.
    fn is_ethereum_fork_active_at_block(&self, fork: EthereumHardfork, block_number: u64) -> bool {
        self.ethereum_fork_activation(fork).active_at_block(block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::Shanghai`] is active at a given
    /// timestamp.
    fn is_shanghai_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.is_ethereum_fork_active_at_timestamp(EthereumHardfork::Shanghai, timestamp)
    }

    /// Convenience method to check if [`EthereumHardfork::Cancun`] is active at a given timestamp.
    fn is_cancun_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.is_ethereum_fork_active_at_timestamp(EthereumHardfork::Cancun, timestamp)
    }

    /// Convenience method to check if [`EthereumHardfork::Prague`] is active at a given timestamp.
    fn is_prague_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.is_ethereum_fork_active_at_timestamp(EthereumHardfork::Prague, timestamp)
    }

    /// Convenience method to check if [`EthereumHardfork::Osaka`] is active at a given timestamp.
    fn is_osaka_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.is_ethereum_fork_active_at_timestamp(EthereumHardfork::Osaka, timestamp)
    }

    /// Convenience method to check if [`EthereumHardfork::Byzantium`] is active at a given block
    /// number.
    fn is_byzantium_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::Byzantium, block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::SpuriousDragon`] is active at a given
    /// block number.
    fn is_spurious_dragon_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::SpuriousDragon, block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::Homestead`] is active at a given block
    /// number.
    fn is_homestead_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::Homestead, block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::London`] is active at a given block
    /// number.
    fn is_london_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::London, block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::Constantinople`] is active at a given
    /// block number.
    fn is_constantinople_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::Constantinople, block_number)
    }

    /// Convenience method to check if [`EthereumHardfork::Paris`] is active at a given block
    /// number.
    fn is_paris_active_at_block(&self, block_number: u64) -> bool {
        self.is_ethereum_fork_active_at_block(EthereumHardfork::Paris, block_number)
    }
}

/// A type allowing to configure activation [`ForkCondition`]s for a given list of
/// [`EthereumHardfork`]s.
#[derive(Debug, Clone)]
pub struct EthereumChainHardforks {
    forks: Vec<(EthereumHardfork, ForkCondition)>,
}

impl EthereumChainHardforks {
    /// Creates a new [`EthereumChainHardforks`] with the given list of forks.
    pub fn new(forks: impl IntoIterator<Item = (EthereumHardfork, ForkCondition)>) -> Self {
        let mut forks = forks.into_iter().collect::<Vec<_>>();
        forks.sort();
        Self { forks }
    }

    /// Creates a new [`EthereumChainHardforks`] with Mainnet configuration.
    pub fn mainnet() -> Self {
        Self::new(EthereumHardfork::mainnet())
    }

    /// Creates a new [`EthereumChainHardforks`] with Sepolia configuration.
    pub fn sepolia() -> Self {
        Self::new(EthereumHardfork::sepolia())
    }

    /// Creates a new [`EthereumChainHardforks`] with Holesky configuration.
    pub fn holesky() -> Self {
        Self::new(EthereumHardfork::holesky())
    }

    /// Creates a new [`EthereumChainHardforks`] with Hoodi configuration.
    pub fn hoodi() -> Self {
        Self::new(EthereumHardfork::hoodi())
    }
}

impl EthereumHardforks for EthereumChainHardforks {
    fn ethereum_fork_activation(&self, fork: EthereumHardfork) -> ForkCondition {
        let Ok(idx) = self.forks.binary_search_by(|(f, _)| f.cmp(&fork)) else {
            return ForkCondition::Never;
        };

        self.forks[idx].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use core::str::FromStr;

    #[test]
    fn check_hardfork_from_str() {
        let hardfork_str = [
            "frOntier",
            "homEstead",
            "dao",
            "tAngerIne",
            "spurIousdrAgon",
            "byzAntium",
            "constantinople",
            "petersburg",
            "istanbul",
            "muirglacier",
            "bErlin",
            "lonDon",
            "arrowglacier",
            "grayglacier",
            "PARIS",
            "ShAnGhAI",
            "CaNcUn",
            "PrAguE",
        ];
        let expected_hardforks = [
            EthereumHardfork::Frontier,
            EthereumHardfork::Homestead,
            EthereumHardfork::Dao,
            EthereumHardfork::Tangerine,
            EthereumHardfork::SpuriousDragon,
            EthereumHardfork::Byzantium,
            EthereumHardfork::Constantinople,
            EthereumHardfork::Petersburg,
            EthereumHardfork::Istanbul,
            EthereumHardfork::MuirGlacier,
            EthereumHardfork::Berlin,
            EthereumHardfork::London,
            EthereumHardfork::ArrowGlacier,
            EthereumHardfork::GrayGlacier,
            EthereumHardfork::Paris,
            EthereumHardfork::Shanghai,
            EthereumHardfork::Cancun,
            EthereumHardfork::Prague,
        ];

        let hardforks: Vec<EthereumHardfork> =
            hardfork_str.iter().map(|h| EthereumHardfork::from_str(h).unwrap()).collect();

        assert_eq!(hardforks, expected_hardforks);
    }

    #[test]
    fn check_nonexistent_hardfork_from_str() {
        assert!(EthereumHardfork::from_str("not a hardfork").is_err());
    }
}
