use alloy_primitives::{BlockNumber, U256};

/// The condition at which a fork is activated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ForkCondition {
    /// The fork is activated after a certain block.
    Block(BlockNumber),
    /// The fork is activated after a total difficulty has been reached.
    TTD {
        /// The activation block number for the merge.
        ///
        /// This should represent the first post-merge block for the given network. Sepolia and
        /// mainnet are the only networks that have merged, and they have both finalized
        /// post-merge, so total difficulty is effectively deprecated.
        activation_block_number: BlockNumber,
        /// The block number at which TTD is reached, if it is known.
        ///
        /// This should **NOT** be set unless you want this block advertised as [EIP-2124][eip2124]
        /// `FORK_NEXT`. This is currently only the case for Sepolia and Holesky.
        ///
        /// [eip2124]: https://eips.ethereum.org/EIPS/eip-2124
        fork_block: Option<BlockNumber>,
        /// The total difficulty after which the fork is activated.
        total_difficulty: U256,
    },
    /// The fork is activated after a specific timestamp.
    Timestamp(u64),
    /// The fork is never activated
    #[default]
    Never,
}

impl ForkCondition {
    /// Block number 0, equivalent to activation at genesis.
    pub const ZERO_BLOCK: Self = Self::Block(0);

    /// Timestamp 0, equivalent to activation at genesis.
    pub const ZERO_TIMESTAMP: Self = Self::Timestamp(0);

    /// Returns true if the fork condition is timestamp based.
    pub const fn is_timestamp(&self) -> bool {
        matches!(self, Self::Timestamp(_))
    }

    /// Returns true if the fork condition is TTD based.
    pub const fn is_ttd(&self) -> bool {
        matches!(self, Self::TTD { .. })
    }

    /// Returns true if the fork condition is block based.
    pub const fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    /// Checks whether the fork condition is satisfied at the given block.
    ///
    /// This will return true if the block number is equal or greater than the activation block of:
    /// - [`ForkCondition::Block`]
    /// - [`ForkCondition::TTD`]
    ///
    /// For timestamp conditions, this will always return false.
    pub const fn active_at_block(&self, current_block: BlockNumber) -> bool {
        matches!(self, Self::Block(block)
        | Self::TTD { activation_block_number: block, .. } if current_block >= *block)
    }

    /// Checks if the given block is the first block that satisfies the fork condition.
    ///
    /// This will return false for any condition that is not block based.
    pub const fn transitions_at_block(&self, current_block: BlockNumber) -> bool {
        matches!(self, Self::Block(block) if current_block == *block)
    }

    /// Checks whether the fork condition is satisfied at the given total difficulty and difficulty
    /// of a current block.
    ///
    /// The fork is considered active if the _previous_ total difficulty is above the threshold.
    /// To achieve that, we subtract the passed `difficulty` from the current block's total
    /// difficulty, and check if it's above the Fork Condition's total difficulty (here:
    /// `58_750_000_000_000_000_000_000`)
    ///
    /// This will return false for any condition that is not TTD-based.
    pub fn active_at_ttd(&self, ttd: U256, difficulty: U256) -> bool {
        matches!(self, Self::TTD { total_difficulty, .. }
            if ttd.saturating_sub(difficulty) >= *total_difficulty)
    }

    /// Checks whether the fork condition is satisfied at the given timestamp.
    ///
    /// This will return false for any condition that is not timestamp-based.
    pub const fn active_at_timestamp(&self, timestamp: u64) -> bool {
        matches!(self, Self::Timestamp(time) if timestamp >= *time)
    }

    /// Checks if the given block is the first block that satisfies the fork condition.
    ///
    /// This will return false for any condition that is not timestamp based.
    pub const fn transitions_at_timestamp(&self, timestamp: u64, parent_timestamp: u64) -> bool {
        matches!(self, Self::Timestamp(time) if timestamp >= *time && parent_timestamp < *time)
    }

    /// Checks whether the fork condition is satisfied at the given timestamp or number.
    pub const fn active_at_timestamp_or_number(&self, timestamp: u64, block_number: u64) -> bool {
        self.active_at_timestamp(timestamp) || self.active_at_block(block_number)
    }

    /// Get the total terminal difficulty for this fork condition.
    ///
    /// Returns `None` for fork conditions that are not TTD based.
    pub const fn ttd(&self) -> Option<U256> {
        match self {
            Self::TTD { total_difficulty, .. } => Some(*total_difficulty),
            _ => None,
        }
    }

    /// Returns the block of the fork condition, if it is block number based, or if it's difficulty
    /// based and the fork block is known.
    pub const fn block_number(&self) -> Option<u64> {
        match self {
            Self::Block(number) => Some(*number),
            Self::TTD { activation_block_number, .. } => Some(*activation_block_number),
            _ => None,
        }
    }

    /// Returns the timestamp of the fork condition, if it is timestamp based.
    pub const fn as_timestamp(&self) -> Option<u64> {
        match self {
            Self::Timestamp(timestamp) => Some(*timestamp),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;

    #[test]
    fn test_active_at_block() {
        // Test if the condition is active at the current block number
        let fork_condition = ForkCondition::Block(10);
        assert!(fork_condition.active_at_block(10), "The condition should be active at block 10");

        // Test if the condition is not active at a lower block number
        assert!(
            !fork_condition.active_at_block(9),
            "The condition should not be active at block 9"
        );

        // Test if TTD-based condition with known block activates
        let fork_condition = ForkCondition::TTD {
            activation_block_number: 10,
            fork_block: Some(10),
            total_difficulty: U256::from(1000),
        };
        assert!(
            fork_condition.active_at_block(10),
            "The TTD condition should be active at block 10"
        );
    }

    #[test]
    fn test_transitions_at_block() {
        // Test if the condition transitions at the correct block number
        let fork_condition = ForkCondition::Block(10);
        assert!(
            fork_condition.transitions_at_block(10),
            "The condition should transition at block 10"
        );

        // Test if the condition does not transition at a different block number
        assert!(
            !fork_condition.transitions_at_block(9),
            "The condition should not transition at a different block number"
        );
        assert!(
            !fork_condition.transitions_at_block(11),
            "The condition should not transition at a different block number"
        );
    }

    #[test]
    fn test_active_at_ttd() {
        // Test if the condition activates at the correct total difficulty
        let fork_condition = ForkCondition::TTD {
            activation_block_number: 10,
            fork_block: Some(10),
            total_difficulty: U256::from(1000),
        };
        assert!(
            fork_condition.active_at_ttd(U256::from(1000000), U256::from(100)),
            "The TTD condition should be active when the total difficulty matches"
        );

        // Test if the condition does not activate when the total difficulty is lower
        assert!(
            !fork_condition.active_at_ttd(U256::from(900), U256::from(100)),
            "The TTD condition should not be active when the total difficulty is lower"
        );

        // Test with a saturated subtraction
        assert!(
            !fork_condition.active_at_ttd(U256::from(900), U256::from(1000)),
            "The TTD condition should not be active when the subtraction saturates"
        );
    }

    #[test]
    fn test_active_at_timestamp() {
        // Test if the condition activates at the correct timestamp
        let fork_condition = ForkCondition::Timestamp(12345);
        assert!(
            fork_condition.active_at_timestamp(12345),
            "The condition should be active at timestamp 12345"
        );

        // Test if the condition does not activate at an earlier timestamp
        assert!(
            !fork_condition.active_at_timestamp(12344),
            "The condition should not be active at an earlier timestamp"
        );
    }

    #[test]
    fn test_transitions_at_timestamp() {
        // Test if the condition transitions at the correct timestamp
        let fork_condition = ForkCondition::Timestamp(12345);
        assert!(
            fork_condition.transitions_at_timestamp(12345, 12344),
            "The condition should transition at timestamp 12345"
        );

        // Test if the condition does not transition if the parent timestamp is already the same
        assert!(
            !fork_condition.transitions_at_timestamp(12345, 12345),
            "The condition should not transition if the parent timestamp is already 12345"
        );
        // Test with earlier timestamp
        assert!(
            !fork_condition.transitions_at_timestamp(123, 122),
            "The condition should not transition if the parent timestamp is earlier"
        );
    }
}
