use crate::packet_buffer::PacketBuffer;
use crate::query_type::QueryType;
use std::io::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
  pub name: String,
  pub qtype: QueryType,
}

impl DnsQuestion {
  pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
    DnsQuestion {
      name: name,
      qtype: qtype,
    }
  }

  pub fn read(&mut self, buffer: &mut PacketBuffer) -> Result<()> {
    buffer.read_domain(&mut self.name)?;
    self.qtype = QueryType::from_num(buffer.read_u16()?); // qtype
    let _ = buffer.read_u16()?; // class

    Ok(())
  }
}
