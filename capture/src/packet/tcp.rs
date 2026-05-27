use std::fmt;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TcpHeader<'a> {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data: &'a [u8],
}

impl TcpHeader<'_> {
    pub const MIN_LEN: usize = 20;
    pub const HEADER: &'static str = "tcp";

    pub fn parse(bytes: &[u8]) -> Result<TcpHeader<'_>, super::PacketError> {
        if bytes.len() < Self::MIN_LEN {
            return Err(super::PacketError::InvalidHeaderLength {
                header: Self::HEADER,
                min: Self::MIN_LEN,
                actual: bytes.len(),
            });
        }

        Ok(TcpHeader {
            src_port: u16::from_be_bytes(bytes[0..=1].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "source port",
                }
            })?),
            dst_port: u16::from_be_bytes(bytes[2..=3].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "destination port",
                }
            })?),
            seq_num: u32::from_be_bytes(bytes[4..=7].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "sequence num",
                }
            })?),
            ack_num: u32::from_be_bytes(bytes[8..=11].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "ack num",
                }
            })?),
            data: &bytes[20..],
        })
    }
}

impl fmt::Display for TcpHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TCP Header:\nSource Port: {}\nDestination Port: {}\nSequence Num: {}\nAck Num: {}",
            self.src_port, self.dst_port, self.seq_num, self.ack_num
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketError;

    #[test]
    fn tcp_header_parse_success() {
        let header: &[u8; _] = &[
            0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F, 0x7A, 0x8B, 0x00, 0x8B, 0x7A, 0x6F, 0x5E,
            0x4D, 0x3C, 0x2B, 0x1A, 0x01, 0x02,
        ];

        let expected = TcpHeader {
            src_port: 0x1A,
            dst_port: 0x2B3C,
            seq_num: 0x4D5E6F7A,
            ack_num: 0x8B008B7A,
            data: &[][..],
        };

        assert_eq!(TcpHeader::parse(header), Ok(expected));
    }

    #[test]
    fn tcp_header_min_len_fail() {
        let header: &[u8] = &[][..];

        let err = PacketError::InvalidHeaderLength {
            header: "tcp",
            min: 20,
            actual: 0,
        };

        assert_eq!(TcpHeader::parse(header), Err(err));
    }
}
