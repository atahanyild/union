use enumorph::Enumorph;
use ibc_solidity::Ibc;
use macros::model;
use subset_of::SubsetOf;
use unionlabs::hash::H256;

#[model]
#[derive(Enumorph, SubsetOf)]
pub enum ModuleCall {
    FetchGetLogs(FetchGetLogs),
    MakeFullEvent(MakeFullEvent),
}

/// Fetch all events in `block_number` emitted by the `IBCHandler` via [`eth_getLogs`].
///
/// [`eth_getLogs`]: https://ethereum.org/en/developers/docs/apis/json-rpc/#[model]th_getlogs
#[model]
pub struct FetchGetLogs {
    pub block_number: u64,
    /// If set, only fetch blocks up to this range; otherwise indefinitely unfold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up_to: Option<u64>,
}

/// Construct a full ChainEvent from the given EVM event and associated metadata.
#[model]
pub struct MakeFullEvent {
    /// The *execution* block number that this event was emitted at.
    pub block_number: u64,
    /// Tx hash of the transaction that emitted this event.
    pub tx_hash: H256,
    pub event: IbcEvents,
}

#[model]
pub enum IbcEvents {
    RegisterClient(Ibc::RegisterClient),
    CreateClient(Ibc::CreateClient),
    UpdateClient(Ibc::UpdateClient),
    ConnectionOpenInit(Ibc::ConnectionOpenInit),
    ConnectionOpenTry(Ibc::ConnectionOpenTry),
    ConnectionOpenAck(Ibc::ConnectionOpenAck),
    ConnectionOpenConfirm(Ibc::ConnectionOpenConfirm),
    ChannelOpenInit(Ibc::ChannelOpenInit),
    ChannelOpenTry(Ibc::ChannelOpenTry),
    ChannelOpenAck(Ibc::ChannelOpenAck),
    ChannelOpenConfirm(Ibc::ChannelOpenConfirm),
    ChannelCloseInit(Ibc::ChannelCloseInit),
    ChannelCloseConfirm(Ibc::ChannelCloseConfirm),
    PacketSend(Ibc::PacketSend),
    PacketRecv(Ibc::PacketRecv),
    IntentPacketRecv(Ibc::IntentPacketRecv),
    WriteAck(Ibc::WriteAck),
    PacketAck(Ibc::PacketAck),
    PacketTimeout(Ibc::PacketTimeout),
}
