use crate::days::Day;
use crate::util::number::parse_binary;

pub const DAY16: Day = Day {
    puzzle1,
    puzzle2,
};

fn puzzle1(input: &String) {
    let packet = Packet::parse(input).unwrap();

    let result = packet.sum_versions();

    println!("Puzzle 1 answer: {}", result);
}

fn puzzle2(input: &String) {
    let packet = Packet::parse(input).unwrap();

    let result = packet.compute().unwrap();

    println!("Puzzle 2 answer: {}", result);
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum PacketData {
    Value(usize),
    SubPackets(Vec<Packet>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    data: PacketData,
}

impl Packet {
    fn parse(input: &str) -> Option<Packet> {
        let binary = decode(input);
        let mut pointer = 0;

        // The input is always one big packet, which contains multiple packets.
        read_packet(&binary, &mut pointer)
    }

    fn sum_versions(&self) -> usize {
        self.version + match &self.data {
            PacketData::Value(_) => 0,
            PacketData::SubPackets(p) => p.iter().map(|p| p.sum_versions()).sum()
        }
    }

    const SUM: usize = 0;
    const MUL: usize = 1;
    const MIN: usize = 2;
    const MAX: usize = 3;
    const LITERAL: usize = 4;
    const GT: usize = 5;
    const LT: usize = 6;
    const EQ: usize = 7;

    fn compute(&self) -> Option<usize> {
        match &self.data {
            PacketData::Value(v) => if self.type_id == Self::LITERAL { Some(*v) } else { None },
            PacketData::SubPackets(packets) => {
                let values: Vec<usize> = packets.iter().filter_map(|p| p.compute()).collect();
                match self.type_id {
                    Self::SUM => Some(values.into_iter().sum()),
                    Self::MUL => values.into_iter().reduce(|a, b| a * b),
                    Self::MIN => values.into_iter().min(),
                    Self::MAX => values.into_iter().max(),
                    Self::GT | Self::LT | Self::EQ => {
                        if values.len() != 2 {
                            None
                        } else {
                            let first = values[0];
                            let second = values[1];
                            match self.type_id {
                                Self::GT => Some(if first > second { 1 } else { 0 }),
                                Self::LT => Some(if first < second { 1 } else { 0 }),
                                Self::EQ => Some(if first == second { 1 } else { 0 }),
                                _ => None
                            }
                        }
                    },
                    _ => None
                }
            }
        }
    }
}

fn read_bit(input: &str, pointer: &mut usize) -> Option<char> {
    let bit = input.chars().nth(*pointer);
    *pointer += 1;
    bit
}

fn read_binary(input: &str, pointer: &mut usize, bits: usize) -> Option<usize> {
    let start = *pointer;
    let end = *pointer + bits;
    if end > input.len() {
        None
    } else {
        let value = parse_binary(&input[start..end]);
        *pointer = end;
        Some(value)
    }
}

fn read_literal(input: &str, pointer: &mut usize) -> Option<usize> {
    let mut continuation = '1';
    let mut result = 0;
    while continuation == '1' {
        continuation = read_bit(input, pointer)?;
        result <<= 4;
        result += read_binary(input, pointer, 4)?;
    }

    Some(result)
}

fn read_packet(input: &str, pointer: &mut usize) -> Option<Packet> {
    // Packets are defined by:
    // 3 bits defining the version
    // 3 bits defining the version
    // 3 bits defining the type ID
    // Data based on the ID:
    // 4 => Literal value
    //      encoded by one continuation bit (1 = continue, 0 = last) and 4 value bits
    //      Note that there are trailing 0's to make the total bits in this packet a multiple of 4
    // _ => operator packet. These will be handled in more detail later
    //      1 bit defining length encoding:
    //          0 = 15 bit number representing number of bits in sub packets
    //          1 = 11 bit number representing number of sub packets
    //      A number of subpackets, see above.
    //      Some amount of trailing 0's that I haven't figured out yet?

    let version = read_binary(input, pointer, 3)?;
    let type_id = read_binary(input, pointer, 3)?;

    let data = match type_id {
        4 => PacketData::Value(read_literal(input, pointer)?),
        _ => {
            PacketData::SubPackets(match read_bit(input, pointer)? {
                '0' => {
                    let sub_packets_bits = read_binary(input, pointer, 15)?;
                    let start = *pointer;
                    let end = *pointer + sub_packets_bits;
                    if end > input.len() { return None; }
                    *pointer = end;
                    let sub_packets_input = &input[start..end];
                    let mut sub_packet_pointer = 0;
                    let mut sub_packets = vec![];
                    while let Some(packet) = read_packet(sub_packets_input, &mut sub_packet_pointer) {
                        sub_packets.push(packet);
                    }
                    sub_packets
                }
                '1' => {
                    let num_sub_packets = read_binary(input, pointer, 11)?;
                    let mut sub_packets = vec![];
                    for _ in 0..num_sub_packets {
                        sub_packets.push(read_packet(input, pointer)?);
                    }
                    sub_packets
                }
                _ => return None
            })
        }
    };

    Some(Packet { version, type_id, data })
}

fn decode(input: &str) -> String {
    // Initially, the input is HEX-encoded binary data.
    // For parsing the packets, it's easier to have the binary string, so we'll be converting the data here.
    input.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Illegal hex char: {}", c)
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::days::day16::{decode, Packet, PacketData, read_bit, read_literal, read_packet};

    #[test]
    fn test_decode() {
        assert_eq!(decode("38006F45291200"), "00111000000000000110111101000101001010010001001000000000");
    }

    #[test]
    fn test_read_packet() {
        assert_eq!(read_packet("110100101111111000101000", &mut 0), Some(Packet {
            version: 6,
            type_id: 4,
            data: PacketData::Value(2021),
        }));

        // Two packets:         |VVVTTTAAAAAVVVTTTBBBBBBBBBB|
        let sub_packets: &str = "110100010100101001000100100";
        let mut sub_packet_pointer = 0;
        assert_eq!(read_packet(sub_packets, &mut sub_packet_pointer), Some(Packet { version: 6, type_id: 4, data: PacketData::Value(10) }));
        assert_eq!(read_packet(sub_packets, &mut sub_packet_pointer), Some(Packet { version: 2, type_id: 4, data: PacketData::Value(20) }));

        // Nested packets type 0
        assert_eq!(read_packet("00111000000000000110111101000101001010010001001000000000", &mut 0), Some(Packet {
            version: 1,
            type_id: 6,
            data: PacketData::SubPackets(vec![
                Packet { version: 6, type_id: 4, data: PacketData::Value(10) },
                Packet { version: 2, type_id: 4, data: PacketData::Value(20) },
            ]),
        }));

        // Nested packets type 1
        assert_eq!(read_packet("11101110000000001101010000001100100000100011000001100000", &mut 0), Some(Packet {
            version: 7,
            type_id: 3,
            data: PacketData::SubPackets(vec![
                Packet { version: 2, type_id: 4, data: PacketData::Value(1) },
                Packet { version: 4, type_id: 4, data: PacketData::Value(2) },
                Packet { version: 1, type_id: 4, data: PacketData::Value(3) },
            ]),
        }))
    }

    #[test]
    fn test_read_bit() {
        let input = "110101";
        let mut pointer = 0;
        assert_eq!(read_bit(input, &mut pointer), Some('1'));
        assert_eq!(read_bit(input, &mut pointer), Some('1'));
        assert_eq!(read_bit(input, &mut pointer), Some('0'));
        assert_eq!(pointer, 3);
    }

    #[test]
    fn test_read_literal() {
        let input = "101111111000101000";
        let mut pointer = 0;
        assert_eq!(read_literal(input, &mut pointer), Some(2021));
        assert_eq!(pointer, 15); // Trailing 0's are not read.

        assert_eq!(read_literal("1000100100", &mut 0), Some(20));
    }

    #[test]
    fn test_parse() {
        // semi-itest for puzzle 1.
        assert_eq!(Packet::parse("8A004A801A8002F478").map(|p| p.sum_versions()), Some(16));
        assert_eq!(Packet::parse("620080001611562C8802118E34").map(|p| p.sum_versions()), Some(12));
        assert_eq!(Packet::parse("C0015000016115A2E0802F182340").map(|p| p.sum_versions()), Some(23));
        assert_eq!(Packet::parse("A0016C880162017C3686B18A3D4780").map(|p| p.sum_versions()), Some(31));
    }

    #[test]
    fn test_compute() {
        assert_eq!(Packet::parse("C200B40A82").and_then(|p| p.compute()), Some(3));
        assert_eq!(Packet::parse("04005AC33890").and_then(|p| p.compute()), Some(54));
        assert_eq!(Packet::parse("880086C3E88112").and_then(|p| p.compute()), Some(7));
        assert_eq!(Packet::parse("CE00C43D881120").and_then(|p| p.compute()), Some(9));
        assert_eq!(Packet::parse("D8005AC2A8F0").and_then(|p| p.compute()), Some(1));
        assert_eq!(Packet::parse("F600BC2D8F").and_then(|p| p.compute()), Some(0));
        assert_eq!(Packet::parse("9C005AC2F8F0").and_then(|p| p.compute()), Some(0));
        assert_eq!(Packet::parse("9C0141080250320F1802104A08").and_then(|p| p.compute()), Some(1));
    }
}