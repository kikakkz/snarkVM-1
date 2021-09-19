// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::MerkleRoot;
use snarkvm_utilities::{FromBytes, ToBytes};

use anyhow::Result;
use rand::{CryptoRng, Rng};

pub trait BlockScheme: Clone + Eq + FromBytes + ToBytes + Send + Sync {
    type BlockHash: Clone + Eq + FromBytes + ToBytes;
    type BlockHeader: Clone + Eq + FromBytes + ToBytes;
    type CommitmentsRoot: Clone + Eq + FromBytes + ToBytes;
    type Transactions: Clone + Eq + FromBytes + ToBytes;
    type Proof: Clone + Eq + FromBytes + ToBytes;

    /// Initializes a new instance of a block.
    fn new<R: Rng + CryptoRng>(
        previous_block_hash: Self::BlockHash,
        transactions: &Self::Transactions,
        commitments_root: Self::CommitmentsRoot,
        serial_numbers_root: MerkleRoot,
        block_height: u32,
        difficulty_target: u64,
        rng: &mut R,
    ) -> Result<Self>;

    /// Returns the previous block hash.
    fn previous_block_hash(&self) -> &Self::BlockHash;

    /// Returns the header.
    fn header(&self) -> &Self::BlockHeader;

    /// Returns the transactions.
    fn transactions(&self) -> &Self::Transactions;

    /// Returns the proof.
    fn proof(&self) -> &Self::Proof;

    /// Returns the hash of this block.
    fn to_hash(&self) -> Result<Self::BlockHash>;
}
