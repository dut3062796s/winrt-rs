use super::RuntimeType;

/// A globally unique identifier [(GUID)](https://docs.microsoft.com/en-us/dotnet/api/system.guid?view=netcore-3.1)
#[repr(C)]
#[derive(Clone, Default, PartialEq)]
pub struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Guid {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

unsafe impl RuntimeType for Guid {
    type Abi = Self;

    fn abi(&self) -> Self::Abi {
        self.clone()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

impl From<&str> for Guid {
    fn from(value: &str) -> Guid {
        assert!(value.len() == 36, "Invalid GUID string");
        let mut bytes = value.bytes();

        let a = ((bytes.next_u32() * 16 + bytes.next_u32()) << 24)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 16)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 8)
            + bytes.next_u32() * 16
            + bytes.next_u32();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let b = ((bytes.next_u16() * 16 + (bytes.next_u16())) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let c = ((bytes.next_u16() * 16 + bytes.next_u16()) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let d = bytes.next_u8() * 16 + bytes.next_u8();
        let e = bytes.next_u8() * 16 + bytes.next_u8();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let f = bytes.next_u8() * 16 + bytes.next_u8();
        let g = bytes.next_u8() * 16 + bytes.next_u8();
        let h = bytes.next_u8() * 16 + bytes.next_u8();
        let i = bytes.next_u8() * 16 + bytes.next_u8();
        let j = bytes.next_u8() * 16 + bytes.next_u8();
        let k = bytes.next_u8() * 16 + bytes.next_u8();

        Guid::from_values(a, b, c, [d, e, f, g, h, i, j, k])
    }
}

trait HexReader {
    fn next_u8(&mut self) -> u8;
    fn next_u16(&mut self) -> u16;
    fn next_u32(&mut self) -> u32;
}

impl HexReader for std::str::Bytes<'_> {
    fn next_u8(&mut self) -> u8 {
        let value = self.next().unwrap();
        match value {
            b'0'..=b'9' => value - b'0',
            b'A'..=b'F' => 10 + value - b'A',
            b'a'..=b'f' => 10 + value - b'a',
            _ => panic!("Invalid GUID string"),
        }
    }

    fn next_u16(&mut self) -> u16 {
        self.next_u8().into()
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u8().into()
    }
}
