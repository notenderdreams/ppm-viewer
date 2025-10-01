
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

impl Encoding {
    pub fn detect_encoding(bytes: &[u8]) -> Self {
        if bytes.len() >= 2 {
            // Little Endian 
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                return Encoding::Utf16Le;
            }
            // Big Endian 
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return Encoding::Utf16Be;
            }
        }

        Encoding::Utf8
    }

    pub fn decode_bytes(bytes: &[u8], encoding: Encoding) -> Result<String, String> {
        match encoding {
            Encoding::Utf8 => String::from_utf8(bytes.to_vec())
                .map_err(|e| format!("UTF-8 decode error: {}", e)),
            Encoding::Utf16Le => {
                let start = if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
                    2
                } else {
                    0
                };

                let vec: Vec<u16> = bytes[start..]
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&vec).map_err(|e| format!("UTF-16LE decode error: {}", e))
            }
            Encoding::Utf16Be => {
                let start = if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
                    2
                } else {
                    0
                };

                let vec: Vec<u16> = bytes[start..]
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&vec).map_err(|e| format!("UTF-16BE decode error: {}", e))
            }
        }
    }
}