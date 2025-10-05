use std::collections::HashMap;

use crate::modules::input_loader::tokens::ScannerStorageKind;

#[derive(Debug)]
pub struct LexProgram {
    pub prelude_code: Vec<u8>,
    pub storage_kind: ScannerStorageKind,
    pub start_condition_inclusive: Vec<Vec<u8>>,
    pub start_condition_exclusive: Vec<Vec<u8>>,
    pub macros: HashMap<Vec<u8>, Vec<u8>>,
    pub user_subroutines: Vec<u8>,
}

impl LexProgram {
    pub fn new() -> Self {
        LexProgram {
            prelude_code: Vec::new(),
            storage_kind: ScannerStorageKind::Pointer,
            start_condition_inclusive: Vec::new(),
            start_condition_exclusive: Vec::new(),
            macros: HashMap::new(),
            user_subroutines: Vec::new(),
        }
    }
}
