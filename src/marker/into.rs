use super::Marker;

impl Into<u8> for Marker {
    fn into(self) -> u8 {
        match self {
            Marker::PositiveFixInt(n) => n,
            Marker::FixMap(n) => 0x80 | (n & 0x0f),
            Marker::FixArray(n) => 0x90 | (n & 0x0f),
        }
    }
}
