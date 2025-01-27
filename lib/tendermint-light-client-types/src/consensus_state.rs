use unionlabs::{
    google::protobuf::timestamp::Timestamp,
    hash::{hash_v2::HexUnprefixed, H256},
    ibc::core::commitment::merkle_root::MerkleRoot,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConsensusState {
    pub timestamp: Timestamp,
    pub root: MerkleRoot,
    pub next_validators_hash: H256<HexUnprefixed>,
}

#[cfg(feature = "ethabi")]
pub mod ethabi {
    use alloy::sol_types::SolValue;
    use unionlabs::impl_ethabi_via_try_from_into;

    use super::*;

    impl_ethabi_via_try_from_into!(ConsensusState => SolConsensusState);

    alloy::sol! {
        struct SolConsensusState {
            uint64 timestamp;
            bytes32 root;
            bytes32 nextValidatorsHash;
        }
    }

    impl From<ConsensusState> for SolConsensusState {
        fn from(value: ConsensusState) -> Self {
            Self {
                timestamp: value.timestamp.as_unix_nanos(),
                root: value.root.hash.get().into(),
                nextValidatorsHash: value.next_validators_hash.get().into(),
            }
        }
    }

    impl From<SolConsensusState> for ConsensusState {
        fn from(value: SolConsensusState) -> Self {
            Self {
                timestamp: Timestamp::try_from_unix_nanos(value.timestamp.into())
                    .expect("impossible"),
                root: H256::new(value.root.0).into(),
                next_validators_hash: H256::new(value.nextValidatorsHash.0),
            }
        }
    }
}

#[cfg(feature = "proto")]
pub mod proto {
    use unionlabs::{
        errors::{InvalidLength, MissingField},
        google::protobuf::timestamp::TryFromTimestampError,
        ibc::core::commitment::merkle_root::TryFromMerkleRootError,
        impl_proto_via_try_from_into, required,
    };

    use crate::ConsensusState;

    impl_proto_via_try_from_into!(ConsensusState => protos::ibc::lightclients::tendermint::v1::ConsensusState);

    impl TryFrom<protos::ibc::lightclients::tendermint::v1::ConsensusState> for ConsensusState {
        type Error = Error;

        fn try_from(
            value: protos::ibc::lightclients::tendermint::v1::ConsensusState,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                timestamp: required!(value.timestamp)?.try_into()?,
                root: required!(value.root)?.try_into()?,
                next_validators_hash: value.next_validators_hash.try_into()?,
            })
        }
    }

    #[derive(Debug, PartialEq, Clone, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        MissingField(#[from] MissingField),
        #[error("invalid root")]
        Root(#[from] TryFromMerkleRootError),
        #[error("invalid next validators hash")]
        NextValidatorsHash(#[from] InvalidLength),
        #[error("invalid timestamp")]
        Timestamp(#[from] TryFromTimestampError),
    }

    impl From<ConsensusState> for protos::ibc::lightclients::tendermint::v1::ConsensusState {
        fn from(value: ConsensusState) -> Self {
            Self {
                timestamp: Some(value.timestamp.into()),
                root: Some(value.root.into()),
                next_validators_hash: value.next_validators_hash.into(),
            }
        }
    }
}
