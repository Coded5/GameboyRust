#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Operands {
    NONE,

    //R8
    A, F, B, C, D, E, H, L,

    //R16
    AF, BC, DE, HL, SP,

    //Addresses
    AddrBC, AddrHL, AddrHLI, AddrHLD, AddrDE, AddrFF00_C,

    //Resolvable Addresses
    AddrFF00_U8,
    AddrU16,

    SP_i8,

    JR_Z,
    JR_NZ,
    JR_C,
    JR_NC,

	H28,
	H00,
	H08,
	H20,
	H18,
	H38,
	H30,
	H10,
    
    U8,
    I8,
    U16,

    I(u8),
}

impl Operands {
    pub fn is_resolvable(&self) -> bool {
        matches!(*self, Operands::AddrFF00_U8 | Operands::AddrU16 | Operands::SP_i8 | Operands::U8 | Operands::I8 | Operands::U16)
    }

    pub fn get_byte_length(&self) -> usize {
        match *self {
            Operands::A => 1,
            Operands::F => 1,
            Operands::B => 1,
            Operands::C => 1,
            Operands::D => 1,
            Operands::E => 1,
            Operands::H => 1,
            Operands::L => 1,
            Operands::AF => 2,
            Operands::BC => 2,
            Operands::DE => 2,
            Operands::HL => 2,
            Operands::SP => 2,
            Operands::AddrBC => 1,
            Operands::AddrHL => 1,
            Operands::AddrHLI => 1,
            Operands::AddrHLD => 1,
            Operands::AddrDE => 1,
            Operands::AddrFF00_C => 1,
            Operands::AddrFF00_U8 => 1,
            Operands::AddrU16 => 1,
            Operands::SP_i8 => 1,
            Operands::JR_Z => 1,
            Operands::JR_NZ => 1,
            Operands::JR_C => 1,
            Operands::JR_NC => 1,
            Operands::H28 => 1,
            Operands::H00 => 1,
            Operands::H08 => 1,
            Operands::H20 => 1,
            Operands::H18 => 1,
            Operands::H38 => 1,
            Operands::H30 => 1,
            Operands::H10 => 1,
            Operands::U8 => 1,
            Operands::I8 => 1,
            Operands::U16 => 2,
            Operands::I(_) => 1,
            Operands::NONE => 0
        }
    }

    pub fn get_str_format(&self, byte: u8, short: u16) -> String {
        match *self {
            Operands::NONE => String::new(),
            Operands::A => String::from("A"),
            Operands::F => String::from("F"),
            Operands::B => String::from("B"),
            Operands::C => String::from("C"),
            Operands::D => String::from("D"),
            Operands::E => String::from("E"),
            Operands::H => String::from("H"),
            Operands::L => String::from("L"),
            Operands::AF => String::from("AF"),
            Operands::BC => String::from("BC"),
            Operands::DE => String::from("DE"),
            Operands::HL => String::from("HL"),
            Operands::SP => String::from("SP"),
            Operands::AddrBC => String::from("(BC)"),
            Operands::AddrHL => String::from("(HL)"),
            Operands::AddrHLI => String::from("(HL+)"),
            Operands::AddrHLD => String::from("(HL-)"),
            Operands::AddrDE => String::from("(DE)"),
            Operands::AddrFF00_C => String::from("(FF00+C)"),
            Operands::AddrFF00_U8 => format!("(FF00+${:02x})", byte),
            Operands::AddrU16 => format!("(${:04x})", short),
            Operands::SP_i8 => format!("SP+${:02x}", byte),
            Operands::JR_Z => String::from("Z"),
            Operands::JR_NZ => String::from("NZ"),
            Operands::JR_C => String::from("C"),
            Operands::JR_NC => String::from("NC"),
            Operands::H28 => String::from("28H"),
            Operands::H00 => String::from("00H"),
            Operands::H08 => String::from("08H"),
            Operands::H20 => String::from("20H"),
            Operands::H18 => String::from("18H"),
            Operands::H38 => String::from("38H"),
            Operands::H30 => String::from("30H"),
            Operands::H10 => String::from("10H"),
            Operands::U8 => format!("${:02x}", byte),
            Operands::I8 => format!("${:02x}", byte),
            Operands::U16 => format!("${:04x}", short),
            Operands::I(x) => x.to_string(),
        }
    }
}
