// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write, Result};

use types::*;
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSubscriptionRequest {
    pub request_header: RequestHeader,
    pub requested_publishing_interval: Double,
    pub requested_lifetime_count: UInt32,
    pub requested_max_keep_alive_count: UInt32,
    pub max_notifications_per_publish: UInt32,
    pub publishing_enabled: Boolean,
    pub priority: Byte,
}

impl MessageInfo for CreateSubscriptionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSubscriptionRequest> for CreateSubscriptionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.requested_publishing_interval.byte_len();
        size += self.requested_lifetime_count.byte_len();
        size += self.requested_max_keep_alive_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.publishing_enabled.byte_len();
        size += self.priority.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> Result<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.requested_publishing_interval.encode(stream)?;
        size += self.requested_lifetime_count.encode(stream)?;
        size += self.requested_max_keep_alive_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += self.priority.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> Result<CreateSubscriptionRequest> {
        let request_header = RequestHeader::decode(stream)?;
        let requested_publishing_interval = Double::decode(stream)?;
        let requested_lifetime_count = UInt32::decode(stream)?;
        let requested_max_keep_alive_count = UInt32::decode(stream)?;
        let max_notifications_per_publish = UInt32::decode(stream)?;
        let publishing_enabled = Boolean::decode(stream)?;
        let priority = Byte::decode(stream)?;
        Ok(CreateSubscriptionRequest {
            request_header: request_header,
            requested_publishing_interval: requested_publishing_interval,
            requested_lifetime_count: requested_lifetime_count,
            requested_max_keep_alive_count: requested_max_keep_alive_count,
            max_notifications_per_publish: max_notifications_per_publish,
            publishing_enabled: publishing_enabled,
            priority: priority,
        })
    }
}
