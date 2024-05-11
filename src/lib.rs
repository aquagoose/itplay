pub struct Track {

}

#[derive(Copy, Clone, PartialEq)]
pub enum Key {
    NoteCut,
    NoteOff,
    C(u8),
    CSharp(u8),
    D(u8),
    DSharp(u8),
    E(u8),
    F(u8),
    FSharp(u8),
    G(u8),
    GSharp(u8),
    A(u8),
    ASharp(u8),
    B(u8)
}

#[derive(Copy, Clone, PartialEq)]
pub struct Note {
    pub key: Key,
    pub instrument: u8
}