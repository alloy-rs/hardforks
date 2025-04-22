#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloy_hardforks::{hardfork, EthereumHardfork, EthereumHardforks, ForkCondition};
pub mod optimism;
pub use optimism::*;

hardfork!(
    /// The name of an optimism hardfork.
    ///
    /// When building a list of hardforks for a chain, it's still expected to mix with
    /// [`EthereumHardfork`].
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    OpHardfork {
        /// Bedrock: <https://blog.oplabs.co/introducing-optimism-bedrock>.
        Bedrock,
        /// Regolith: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#regolith>.
        Regolith,
        /// <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#canyon>.
        Canyon,
        /// Ecotone: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#ecotone>.
        Ecotone,
        /// Fjord: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#fjord>
        Fjord,
        /// Granite: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#granite>
        Granite,
        /// Holocene: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/superchain-upgrades.md#holocene>
        Holocene,
        /// Isthmus: <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/isthmus/overview.md>
        Isthmus,
        /// TODO: add interop hardfork overview when available
        Interop,
    }
);

impl OpHardfork {
    /// Optimism mainnet list of hardforks.
    pub const fn op_mainnet() -> [(Self, ForkCondition); 7] {
        [
            (Self::Bedrock, ForkCondition::Block(OP_MAINNET_BEDROCK_BLOCK)),
            (Self::Regolith, ForkCondition::Timestamp(0)),
            (Self::Canyon, ForkCondition::Timestamp(OP_MAINNET_CANYON_TIMESTAMP)),
            (Self::Ecotone, ForkCondition::Timestamp(OP_MAINNET_ECOTONE_TIMESTAMP)),
            (Self::Fjord, ForkCondition::Timestamp(OP_MAINNET_FJORD_TIMESTAMP)),
            (Self::Granite, ForkCondition::Timestamp(OP_MAINNET_GRANITE_TIMESTAMP)),
            (Self::Holocene, ForkCondition::Timestamp(OP_MAINNET_HOLOCENE_TIMESTAMP)),
        ]
    }

    /// Optimism Sepolia list of hardforks.
    pub const fn op_sepolia() -> [(Self, ForkCondition); 8] {
        [
            (Self::Bedrock, ForkCondition::Block(0)),
            (Self::Regolith, ForkCondition::Timestamp(0)),
            (Self::Canyon, ForkCondition::Timestamp(1699981200)),
            (Self::Ecotone, ForkCondition::Timestamp(1708534800)),
            (Self::Fjord, ForkCondition::Timestamp(1716998400)),
            (Self::Granite, ForkCondition::Timestamp(1723478400)),
            (Self::Holocene, ForkCondition::Timestamp(1732633200)),
            (Self::Isthmus, ForkCondition::Timestamp(1744905600)),
        ]
    }

    /// Base mainnet list of hardforks.
    pub const fn base_mainnet() -> [(Self, ForkCondition); 7] {
        [
            (Self::Bedrock, ForkCondition::Block(0)),
            (Self::Regolith, ForkCondition::Timestamp(0)),
            (Self::Canyon, ForkCondition::Timestamp(1704992401)),
            (Self::Ecotone, ForkCondition::Timestamp(1710374401)),
            (Self::Fjord, ForkCondition::Timestamp(1720627201)),
            (Self::Granite, ForkCondition::Timestamp(1726070401)),
            (Self::Holocene, ForkCondition::Timestamp(1736445601)),
        ]
    }

    /// Base Sepolia list of hardforks.
    pub const fn base_sepolia() -> [(Self, ForkCondition); 8] {
        [
            (Self::Bedrock, ForkCondition::Block(0)),
            (Self::Regolith, ForkCondition::Timestamp(0)),
            (Self::Canyon, ForkCondition::Timestamp(1699981200)),
            (Self::Ecotone, ForkCondition::Timestamp(1708534800)),
            (Self::Fjord, ForkCondition::Timestamp(1716998400)),
            (Self::Granite, ForkCondition::Timestamp(1723478400)),
            (Self::Holocene, ForkCondition::Timestamp(1732633200)),
            (Self::Isthmus, ForkCondition::Timestamp(1744905600)),
        ]
    }
}

/// Extends [`EthereumHardforks`] with optimism helper methods.
#[auto_impl::auto_impl(&, Arc)]
pub trait OpHardforks: EthereumHardforks {
    /// Retrieves [`ForkCondition`] by an [`OpHardfork`]. If `fork` is not present, returns
    /// [`ForkCondition::Never`].
    fn op_fork_activation(&self, fork: OpHardfork) -> ForkCondition;

    /// Convenience method to check if [`OpHardfork::Bedrock`] is active at a given block
    /// number.
    fn is_bedrock_active_at_block(&self, block_number: u64) -> bool {
        self.op_fork_activation(OpHardfork::Bedrock).active_at_block(block_number)
    }

    /// Returns `true` if [`Regolith`](OpHardfork::Regolith) is active at given block
    /// timestamp.
    fn is_regolith_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Regolith).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Canyon`](OpHardfork::Canyon) is active at given block timestamp.
    fn is_canyon_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Canyon).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Ecotone`](OpHardfork::Ecotone) is active at given block timestamp.
    fn is_ecotone_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Ecotone).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Fjord`](OpHardfork::Fjord) is active at given block timestamp.
    fn is_fjord_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Fjord).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Granite`](OpHardfork::Granite) is active at given block timestamp.
    fn is_granite_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Granite).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Holocene`](OpHardfork::Holocene) is active at given block
    /// timestamp.
    fn is_holocene_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Holocene).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Isthmus`](OpHardfork::Isthmus) is active at given block
    /// timestamp.
    fn is_isthmus_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Isthmus).active_at_timestamp(timestamp)
    }

    /// Returns `true` if [`Interop`](OpHardfork::Interop) is active at given block
    /// timestamp.
    fn is_interop_active_at_timestamp(&self, timestamp: u64) -> bool {
        self.op_fork_activation(OpHardfork::Interop).active_at_timestamp(timestamp)
    }
}

/// A type allowing to configure activation [`ForkCondition`]s for a given list of
/// [`OpHardfork`]s.
#[derive(Debug, Clone)]
pub struct OpChainHardforks {
    /// Special case for OP mainnet which had Bedrock activated separately without an associated
    /// [`OpHardfork`].
    berlin_block: Option<u64>,
    /// OP hardfork activations.
    forks: Vec<(OpHardfork, ForkCondition)>,
}

impl OpChainHardforks {
    /// Creates a new [`OpChainHardforks`] with the given list of forks.
    pub fn new(
        forks: impl IntoIterator<Item = (OpHardfork, ForkCondition)>,
        berlin_block: Option<u64>,
    ) -> Self {
        let mut forks = forks.into_iter().collect::<Vec<_>>();
        forks.sort();
        Self { forks, berlin_block }
    }

    /// Creates a new [`OpChainHardforks`] with OP mainnet configuration.
    pub fn op_mainnet() -> Self {
        Self::new(OpHardfork::op_mainnet(), Some(3950000))
    }

    /// Creates a new [`OpChainHardforks`] with OP Sepolia configuration.
    pub fn op_sepolia() -> Self {
        Self::new(OpHardfork::op_sepolia(), None)
    }

    /// Creates a new [`OpChainHardforks`] with Base mainnet configuration.
    pub fn base_mainnet() -> Self {
        Self::new(OpHardfork::base_mainnet(), None)
    }

    /// Creates a new [`OpChainHardforks`] with Base Sepolia configuration.
    pub fn base_sepolia() -> Self {
        Self::new(OpHardfork::base_sepolia(), None)
    }
}

impl EthereumHardforks for OpChainHardforks {
    fn ethereum_fork_activation(&self, fork: EthereumHardfork) -> ForkCondition {
        if fork < EthereumHardfork::Berlin {
            // We assume that OP chains were launched with all forks before Berlin activated.
            ForkCondition::Block(0)
        } else if fork == EthereumHardfork::Berlin {
            // Handle special OP mainnet case of Berlin activation.
            // If `berlin_block` is not set, assume it was enabled at genesis.
            self.berlin_block.map_or(ForkCondition::Block(0), ForkCondition::Block)
        } else if fork <= EthereumHardfork::Paris {
            // Bedrock activates all hardforks up to Paris.
            self.op_fork_activation(OpHardfork::Bedrock)
        } else if fork <= EthereumHardfork::Shanghai {
            // Canyon activates Shanghai hardfork.
            self.op_fork_activation(OpHardfork::Canyon)
        } else if fork <= EthereumHardfork::Cancun {
            // Ecotone activates Cancun hardfork.
            self.op_fork_activation(OpHardfork::Ecotone)
        } else if fork <= EthereumHardfork::Prague {
            // Isthmus activates Prague hardfork.
            self.op_fork_activation(OpHardfork::Isthmus)
        } else {
            ForkCondition::Never
        }
    }
}

impl OpHardforks for OpChainHardforks {
    fn op_fork_activation(&self, fork: OpHardfork) -> ForkCondition {
        let Ok(idx) = self.forks.binary_search_by(|(f, _)| f.cmp(&fork)) else {
            return ForkCondition::Never;
        };

        self.forks[idx].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    extern crate alloc;

    #[test]
    fn check_op_hardfork_from_str() {
        let hardfork_str = [
            "beDrOck", "rEgOlITH", "cAnYoN", "eCoToNe", "FJorD", "GRaNiTe", "hOlOcEnE", "isthMUS",
            "inTerOP",
        ];
        let expected_hardforks = [
            OpHardfork::Bedrock,
            OpHardfork::Regolith,
            OpHardfork::Canyon,
            OpHardfork::Ecotone,
            OpHardfork::Fjord,
            OpHardfork::Granite,
            OpHardfork::Holocene,
            OpHardfork::Isthmus,
            OpHardfork::Interop,
        ];

        let hardforks: alloc::vec::Vec<OpHardfork> =
            hardfork_str.iter().map(|h| OpHardfork::from_str(h).unwrap()).collect();

        assert_eq!(hardforks, expected_hardforks);
    }

    #[test]
    fn check_nonexistent_hardfork_from_str() {
        assert!(OpHardfork::from_str("not a hardfork").is_err());
    }
}
