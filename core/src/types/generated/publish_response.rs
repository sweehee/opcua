// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write, Result};

use types::*;
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PublishResponse {
    pub response_header: ResponseHeader,
    pub subscription_id: UInt32,
    pub available_sequence_numbers: Option<Vec<UInt32>>,
    pub more_notifications: Boolean,
    pub notification_message: NotificationMessage,
    pub results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for PublishResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::PublishResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<PublishResponse> for PublishResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.subscription_id.byte_len();
        size += byte_len_array(&self.available_sequence_numbers);
        size += self.more_notifications.byte_len();
        size += self.notification_message.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> Result<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += write_array(stream, &self.available_sequence_numbers)?;
        size += self.more_notifications.encode(stream)?;
        size += self.notification_message.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> Result<PublishResponse> {
        let response_header = ResponseHeader::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let available_sequence_numbers: Option<Vec<UInt32>> = read_array(stream)?;
        let more_notifications = Boolean::decode(stream)?;
        let notification_message = NotificationMessage::decode(stream)?;
        let results: Option<Vec<StatusCode>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(PublishResponse {
            response_header: response_header,
            subscription_id: subscription_id,
            available_sequence_numbers: available_sequence_numbers,
            more_notifications: more_notifications,
            notification_message: notification_message,
            results: results,
            diagnostic_infos: diagnostic_infos,
        })
    }
}
