/// CTrader Open API ProtoMessage Codec
use prost::{
    bytes::{Buf, BufMut, BytesMut},
    Message,
};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

use crate::protos::spotware_message::ProtoMessage;

//
// # ProtoMessages
// The network communication in Open API 2.0 is performed by means of ProtoMessage objects - the protobuf messages designed by Spotware.
// In order to deal with the network fragmentation we will send messages to the network using the following frame structure:
// +--------------------------+-----------------------------------------+
// | Message Length (4 bytes) | Serialized ProtoMessage object (byte[]) |
// +--------------------------+-----------------------------------------+
//                            |<---------- Message Length ------------->|
//
// # ProtoMessage has the following structure:
// +----------------------+
// | int32 payloadType    |
// | byte[] payload       |
// | string clientMsgId   |
// +----------------------+
// It contains 2 mandatory fields:
// **payloadType**: Contains the ProtoPayloadType ID, This field tells us what is the type of the protobuf object serialized in the second field.
// **payload**: Serialized protobuf message that corresponds to payloadType.
// **clientMsgId**: Request message ID, assigned by the client that will be returned in the response.
// See OpenApiCommonMessages/ProtoMessage

#[derive(Debug, Default)]
pub struct MsgCodec {
    len: Option<usize>,
}

impl Encoder<ProtoMessage> for MsgCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ProtoMessage, buf: &mut BytesMut) -> io::Result<()> {
        let size = msg.encoded_len();
        buf.put_u32(size as u32);

        msg.encode(buf).map_err(io::Error::from)?;
        Ok(())
    }
}

impl Decoder for MsgCodec {
    type Item = ProtoMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(msg_len) = self.len {
            if buf.remaining() >= msg_len {
                let msg = ProtoMessage::decode(buf.split_to(msg_len)).map_err(io::Error::from)?;
                self.len = None;
                Ok(Some(msg))
            } else {
                Ok(None)
            }
        } else if buf.remaining() > 4 {
            let msg_len = buf.split_to(4).get_u32() as usize;
            self.len = Some(msg_len);
            self.decode(buf)
        } else {
            Ok(None)
        }
    }
}
