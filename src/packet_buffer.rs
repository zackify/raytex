use std::io::{Error, ErrorKind, Result};

pub struct PacketBuffer {
  pub buf: [u8; 512],
  pub pos: usize,
}

impl PacketBuffer {
  pub fn new() -> PacketBuffer {
    PacketBuffer {
      buf: [0; 512],
      pos: 0,
    }
  }

  pub fn pos(&self) -> usize {
    self.pos
  }

  pub fn step(&mut self, steps: usize) -> Result<()> {
    self.pos += steps;
    Ok(())
  }

  pub fn seek(&mut self, pos: usize) -> Result<()> {
    self.pos = pos;
    Ok(())
  }

  pub fn read(&mut self) -> Result<u8> {
    if self.pos >= 512 {
      return Err(Error::new(ErrorKind::Other, "End of buffer"));
    }
    let res = self.buf[self.pos];
    self.pos += 1;

    Ok(res)
  }

  pub fn read_u16(&mut self) -> Result<u16> {
    let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

    Ok(res)
  }

  /// Read four bytes, stepping four steps forward
  pub fn read_u32(&mut self) -> Result<u32> {
    let res = ((self.read()? as u32) << 24)
      | ((self.read()? as u32) << 16)
      | ((self.read()? as u32) << 8)
      | ((self.read()? as u32) << 0);

    Ok(res)
  }

  /// Get a single byte, without changing the buffer position
  fn get(&mut self, pos: usize) -> Result<u8> {
    if pos >= 512 {
      return Err(Error::new(ErrorKind::Other, "End of buffer"));
    }
    Ok(self.buf[pos])
  }

  /// Get a range of bytes
  fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
    if start + len >= 512 {
      return Err(Error::new(ErrorKind::Other, "End of buffer"));
    }
    Ok(&self.buf[start..start + len as usize])
  }

  pub fn read_domain(&mut self, outstr: &mut String) -> Result<()> {
    //track current position, so that we can seek back after a jump
    let mut current_position = self.pos();

    let mut jumped = false;
    let max_jumps = 5;
    let mut jumps_performed = 0;

    let mut delimeter = "";

    loop {
      if jumps_performed > max_jumps {
        return Err(Error::new(
          ErrorKind::Other,
          "Max jumps exceeded, something fishy is going on here >:(",
        ));
      }

      let lengthOfLabel = self.get(current_position)?;

      //if this current byte (8 bits) has the first two bits set, then we need to jump to get the name
      if (lengthOfLabel & 0xC0) == 0xC0 {
        //if we havent jumped yet (this is the first time in the loop), then we need to go forward two bytes
        if !jumped {
          self.seek(current_position + 2)?;
        }

        //read another byte, calculate the offset and perform the jump
        let b2 = self.get(current_position + 1)? as u16;

        let offset = (((lengthOfLabel as u16) ^ 0xC0) << 8) | b2;
        current_position = offset as usize;

        jumped = true;
        jumps_performed += 1;
        continue;
      } else {
        // move to the next bytw, this is a normal parsing
        current_position += 1;

        // Domain names are terminated by an empty label of length 0,
        // so if the length is zero we're done.
        if lengthOfLabel == 0 {
          break;
        }
        // Append the delimiter to our output buffer first.
        outstr.push_str(delimeter);

        // Extract the actual ASCII bytes for this label and append them
        // to the output buffer.
        let str_buffer = self.get_range(current_position, lengthOfLabel as usize)?;
        outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

        delimeter = ".";

        // Move forward the full length of the label.
        current_position += lengthOfLabel as usize;
      }
    }
    if !jumped {
      self.seek(current_position)?;
    }
    Ok(())
  }
}
