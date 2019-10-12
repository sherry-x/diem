// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::common::Round;
use crypto::{
    hash::{CryptoHash, CryptoHasher, VoteDataHasher},
    HashValue,
};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
};

// Internal use only. Contains all the fields in VoteDataSerializer that contributes to the
// computation of its hash.
#[derive(Serialize, Deserialize)]
struct VoteDataSerializer {
    block_id: HashValue,
    executed_state_id: HashValue,
    round: Round,
    parent_block_id: HashValue,
    parent_block_round: Round,
}

impl CryptoHash for VoteDataSerializer {
    type Hasher = VoteDataHasher;

    fn hash(&self) -> HashValue {
        let mut state = Self::Hasher::default();
        state.write(lcs::to_bytes(self).expect("Should serialize.").as_ref());
        state.finish()
    }
}

/// VoteData keeps the information about the block, and its parent.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct VoteData {
    /// The id of the proposed block.
    block_id: HashValue,
    /// The id of the state generated by the StateExecutor after executing the proposed block.
    executed_state_id: HashValue,
    /// The round of the block.
    round: Round,
    /// The id of the parent block of the proposal
    parent_block_id: HashValue,
    /// The round of the parent block of the proposal
    parent_block_round: Round,
}

impl Display for VoteData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "VoteData: [block id: {}, round: {:02}, parent_block_id: {}, \
             parent_block_round: {:02}]",
            self.block_id, self.round, self.parent_block_id, self.parent_block_round,
        )
    }
}

impl VoteData {
    pub fn new(
        block_id: HashValue,
        executed_state_id: HashValue,
        round: Round,
        parent_block_id: HashValue,
        parent_block_round: Round,
    ) -> Self {
        Self {
            block_id,
            executed_state_id,
            round,
            parent_block_id,
            parent_block_round,
        }
    }

    /// Return the id of a block that is being voted.
    pub fn block_id(&self) -> HashValue {
        self.block_id
    }

    /// Return the executed state of the proposed block
    pub fn executed_state_id(&self) -> HashValue {
        self.executed_state_id
    }

    /// Return the round of the block
    pub fn block_round(&self) -> Round {
        self.round
    }

    /// Return the id of the parent of the proposed block
    pub fn parent_block_id(&self) -> HashValue {
        self.parent_block_id
    }

    /// Return the round of the parent block of the proposed block
    pub fn parent_block_round(&self) -> Round {
        self.parent_block_round
    }

    /// Return the hash of this struct
    pub fn hash(&self) -> HashValue {
        Self::vote_digest(
            self.block_id,
            self.executed_state_id,
            self.round,
            self.parent_block_id,
            self.parent_block_round,
        )
    }

    /// Return a digest of the vote
    pub fn vote_digest(
        block_id: HashValue,
        executed_state_id: HashValue,
        round: Round,
        parent_block_id: HashValue,
        parent_block_round: Round,
    ) -> HashValue {
        VoteDataSerializer {
            block_id,
            executed_state_id,
            round,
            parent_block_id,
            parent_block_round,
        }
        .hash()
    }
}

impl TryFrom<network::proto::VoteData> for VoteData {
    type Error = failure::Error;

    fn try_from(proto: network::proto::VoteData) -> failure::Result<Self> {
        let block_id = HashValue::from_slice(proto.block_id.as_ref())?;
        let round = proto.round;
        let executed_state_id = HashValue::from_slice(proto.executed_state_id.as_ref())?;
        let parent_block_id = HashValue::from_slice(proto.parent_block_id.as_ref())?;
        let parent_block_round = proto.parent_block_round;
        Ok(VoteData {
            block_id,
            executed_state_id,
            round,
            parent_block_id,
            parent_block_round,
        })
    }
}

impl From<VoteData> for network::proto::VoteData {
    fn from(vote: VoteData) -> Self {
        Self {
            block_id: vote.block_id.to_vec(),
            executed_state_id: vote.executed_state_id.to_vec(),
            round: vote.round,
            parent_block_id: vote.parent_block_id.to_vec(),
            parent_block_round: vote.parent_block_round,
        }
    }
}
