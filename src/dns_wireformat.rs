use crate::dns::{DnsHeader, DnsMessage, DnsQuestion, ResourceRecord};

/// The encoder/decoder for a DNS Message

pub trait WireFormat {
    /// Encode self as an array of bytes to send to servers
    fn encode(&self) -> Vec<u8>;

    /// Decode response from servers from bytes to Self
    fn decode(bytes: &[u8]) -> Self;
}

impl WireFormat for DnsMessage {
    fn encode(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend(self.header.encode());
        res.extend(self.question.iter().flat_map(|q| q.encode()));
        res.extend(self.answer.iter().flat_map(|a| a.encode()));
        res.extend(self.authority.iter().flat_map(|a| a.encode()));
        res.extend(self.additional.iter().flat_map(|a| a.encode()));
        res
    }

    fn decode(bytes: &[u8]) -> Self {
        todo!("Not implemented")
    }
}

impl WireFormat for DnsHeader {
    fn encode(&self) -> Vec<u8> {
        todo!("Not implemented")
    }

    fn decode(bytes: &[u8]) -> Self {
        todo!("Not implemented")
    }
}

impl WireFormat for DnsQuestion {
    fn encode(&self) -> Vec<u8> {
        todo!("Not implemented")
    }

    fn decode(bytes: &[u8]) -> Self {
        todo!("Not implemented")
    }
}

impl WireFormat for ResourceRecord {
    fn encode(&self) -> Vec<u8> {
        todo!("Not implemented")
    }

    fn decode(bytes: &[u8]) -> Self {
        todo!("Not implemented")
    }
}
