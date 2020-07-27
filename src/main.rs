mod dns_header;
mod dns_packet;
mod dns_question;
mod dns_record;
mod packet_buffer;
mod query_type;
mod result_codes;
use dns_packet::DnsPacket;
use packet_buffer::PacketBuffer;

use std::fs::File;
use std::io::Read;
use std::io::Result;

fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = PacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
