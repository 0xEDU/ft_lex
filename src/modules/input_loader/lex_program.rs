use crate::modules::input_loader::tokens::ScannerStorageKind;

#[derive(Debug)]
pub struct LexProgram {
    pub preludeCode: Vec<u8>,
    pub storageKind: ScannerStorageKind,
    pub startConditionInclusive: Vec<Vec<u8>>,
}

impl LexProgram {
    pub fn new() -> Self {
        LexProgram {
            preludeCode: Vec::new(),
            storageKind: ScannerStorageKind::Pointer,
            startConditionInclusive: Vec::new(),
        }
    }
}
