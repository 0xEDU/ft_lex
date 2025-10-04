#[derive(Debug, PartialEq)]
pub enum ScannerStorageKind {
    Array,
    Pointer
}

#[derive(Debug, PartialEq)]
pub enum Token {
    CodeLine(Vec<u8>),
    CodeBlock(Vec<u8>),
    StartConditionInclusive(Vec<u8>),
    StartConditionExclusive(Vec<u8>),
    ScannerStorage(ScannerStorageKind),
    MacroDefinition(Vec<u8>),
    Rule(Vec<u8>),
    UserSubroutine(Vec<u8>),
}