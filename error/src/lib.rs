pub enum Error {
    IndexArrayOutOfBound(usize),
    InvalidOpCode(u8)
}