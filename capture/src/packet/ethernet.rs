use std::fmt;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub(super) struct EthernetHeader<'a> {
    pub(super) dst: [u8; 6],
    pub(super) src: [u8; 6],
    pub(super) ether_type: EtherType,
    pub(super) payload: &'a [u8],
}
#[derive(Debug, PartialEq)]
pub enum EtherType {
    IPv4,
    IPv6,
}

impl EthernetHeader<'_> {
    const MIN_LEN: usize = 14;
    const HEADER: &'static str = "eth";

    pub(super) fn parse(bytes: &[u8]) -> Result<EthernetHeader<'_>, super::PacketError> {
        if bytes.len() < Self::MIN_LEN {
            return Err(super::PacketError::InvalidHeaderLength {
                header: Self::HEADER,
                min: Self::MIN_LEN,
                actual: bytes.len(),
            });
        }
        Ok(EthernetHeader {
            dst: bytes[0..=5].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "destination MAC address",
                }
            })?,
            src: bytes[6..=11].try_into().map_err(|_| {
                super::PacketError::ErrorParsingHeaderFields {
                    header: Self::HEADER,
                    field: "source MAC address",
                }
            })?,
            ether_type: match ((bytes[12] as u16) << 8) + bytes[13] as u16 {
                0x0800 => EtherType::IPv4,
                0x86DD => EtherType::IPv6,
                other => {
                    return Err(super::PacketError::UnsupportedFieldType {
                        header: "ip",
                        field: format!("0x{:04X}", other),
                    });
                }
            },
            payload: &bytes[14..],
        })
    }
}

impl fmt::Display for EthernetHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EthHeader:\nDestination MAC: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nSource MAC: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nEthernet Type: {:?}\n",
            self.dst[0],
            self.dst[1],
            self.dst[2],
            self.dst[3],
            self.dst[4],
            self.dst[5],
            self.src[0],
            self.src[1],
            self.src[2],
            self.src[3],
            self.src[4],
            self.src[5],
            self.ether_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::PacketError;

    #[test]
    fn eth_header_parse_success() {
        let header: &[u8; _] = &[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x00, 0x5E, 0x4D, 0x3C, 0x2B, 0x1A, 0x08, 0x00];
        let expected = EthernetHeader {
            dst: [0, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
            src: [0, 0x5E, 0x4D, 0x3C, 0x2B, 0x1A],
            ether_type: EtherType::IPv4,
            payload: &[][..],
        };
        assert_eq!(EthernetHeader::parse(header), Ok(expected));
    }

    #[test]
    fn eth_header_min_length_fail() {
        let header: &[u8] = &[][..];
        let err = PacketError::InvalidHeaderLength {
            header: "eth",
            min: 14,
            actual: 0,
        };
        assert_eq!(EthernetHeader::parse(header), Err(err));
    }

    #[test]
    fn eth_wrong_eth_type() {
        let header: &[u8; _] = &[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x00, 0x5E, 0x4D, 0x3C, 0x2B, 0x1A, 0x07, 0x00];

        let err = PacketError::UnsupportedFieldType {
            header: "ip",
            field: String::from("0x0700"),
        };
        assert_eq!(EthernetHeader::parse(header), Err(err));
    }
}
