use crate::{hardfork, ForkCondition};
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
            Self::Frontier => Some(0),
            Self::Homestead => Some(1150000),
            Self::Dao => Some(1920000),
            Self::Tangerine => Some(2463000),
            Self::SpuriousDragon => Some(2675000),
            Self::Byzantium => Some(4370000),
            Self::Constantinople | Self::Petersburg => Some(7280000),
            Self::Istanbul => Some(9069000),
            Self::MuirGlacier => Some(9200000),
            Self::Berlin => Some(12244000),
            Self::London => Some(12965000),
            Self::ArrowGlacier => Some(13773000),
            Self::GrayGlacier => Some(15050000),
            Self::Paris => Some(15537394),
            Self::Shanghai => Some(17034870),
            Self::Cancun => Some(19426587),
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
            Self::Dao
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
            Self::Dao
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
            Self::Frontier => Some(1438226773),
            Self::Homestead => Some(1457938193),
            Self::Dao => Some(1468977640),
            Self::Tangerine => Some(1476753571),
            Self::SpuriousDragon => Some(1479788144),
            Self::Byzantium => Some(1508131331),
            Self::Constantinople | Self::Petersburg => Some(1551340324),
            Self::Istanbul => Some(1575807909),
            Self::MuirGlacier => Some(1577953849),
            Self::Berlin => Some(1618481223),
            Self::London => Some(1628166822),
            Self::ArrowGlacier => Some(1639036523),
            Self::GrayGlacier => Some(1656586444),
            Self::Paris => Some(1663224162),
            Self::Shanghai => Some(1681338455),
            Self::Cancun => Some(1710338135),

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
            Self::Shanghai => Some(1696000704),
            Self::Cancun => Some(1707305664),
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
    pub const fn mainnet() -> [(Self, ForkCondition); 17] {
        [
            (Self::Frontier, ForkCondition::Block(0)),
            (Self::Homestead, ForkCondition::Block(1150000)),
            (Self::Dao, ForkCondition::Block(1920000)),
            (Self::Tangerine, ForkCondition::Block(2463000)),
            (Self::SpuriousDragon, ForkCondition::Block(2675000)),
            (Self::Byzantium, ForkCondition::Block(4370000)),
            (Self::Constantinople, ForkCondition::Block(7280000)),
            (Self::Petersburg, ForkCondition::Block(7280000)),
            (Self::Istanbul, ForkCondition::Block(9069000)),
            (Self::MuirGlacier, ForkCondition::Block(9200000)),
            (Self::Berlin, ForkCondition::Block(12244000)),
            (Self::London, ForkCondition::Block(12965000)),
            (Self::ArrowGlacier, ForkCondition::Block(13773000)),
            (Self::GrayGlacier, ForkCondition::Block(15050000)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: 15537394,
                    fork_block: None,
                    total_difficulty: uint!(58_750_000_000_000_000_000_000_U256),
                },
            ),
            (Self::Shanghai, ForkCondition::Timestamp(1681338455)),
            (Self::Cancun, ForkCondition::Timestamp(1710338135)),
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
