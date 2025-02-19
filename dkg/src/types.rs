// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto_derive::CryptoHasher;
use aptos_enum_conversion_derive::EnumConversion;
use aptos_reliable_broadcast::RBMessage;
pub use aptos_types::dkg::DKGAggNode;
use serde::{Deserialize, Serialize};

/// Once DKG starts, a validator should send this message to peers in order to collect DKG transcripts from peers.
#[derive(Clone, Serialize, Deserialize, CryptoHasher, Debug, PartialEq)]
pub struct DKGNodeRequest {
    dealer_epoch: u64,
}

/// Contains a DKG transcript and some metadata.
#[derive(Clone, Serialize, Deserialize, CryptoHasher, Debug, PartialEq)]
pub struct DKGNode {
    //TODO
}

/// The DKG network message.
#[derive(Clone, Serialize, Deserialize, Debug, EnumConversion, PartialEq)]
pub enum DKGMessage {
    NodeRequest(DKGNodeRequest),
    NodeResponse(DKGNode),
}

impl RBMessage for DKGMessage {}
