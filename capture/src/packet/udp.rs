use std::fmt;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct UdpHeader<'a> {
    pub src_port: u16,
    pub dst_port: u16,
    pub data: &'a [u8],
}

impl UdpHeader<'_> {
    pub const MIN_LEN: usize = 8;
    pub const HEADER: &'static str = "udp";

    pub fn parse(bytes: &[u8]) -> Result<UdpHeader<'_>, super::PacketError> {
        if bytes.len() < Self::MIN_LEN {
            return Err(super::PacketError::InvalidHeaderLength {
                header: Self::HEADER,
                min: Self::MIN_LEN,
                actual: bytes.len(),
            });
        }
        Ok(UdpHeader {
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
            data: &bytes[8..],
        })
    }
}

impl fmt::Display for UdpHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UDP Header:\nSource Port: {}\nDestination Port: {}",
            self.src_port, self.dst_port
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketError;

    #[test]
    fn udp_parse_success() {
        let header: &[u8; _] = &[0x00, 0x1A, 0x2B, 0x3C, 0x00, 0x3C, 0x2B, 0x1A];

        let expected = UdpHeader {
            src_port: 0x001A,
            dst_port: 0x2B3C,
            data: &[][..],
        };

        assert_eq!(UdpHeader::parse(header), Ok(expected));
    }

    #[test]
    fn udp_min_len_fail() {
        let header: &[u8] = &[][..];
        let err = PacketError::InvalidHeaderLength {
            header: "udp",
            min: 8,
            actual: 0,
        };

        assert_eq!(UdpHeader::parse(header), Err(err));
    }
}
