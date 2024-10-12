use super::*;
mod shell_functions;
mod shell_function_table;
mod interpreter_functions;
mod interpreter_function_table;
pub use interpreter_functions::InterpreterFunctions;
pub use shell_functions::ShellFunctions;


pub struct FunctionEntry<T: FunctionEnum> {
    function_number: T,     // The identifier used to select the execution code
    function_name: &'static str, 
}


impl<T: FunctionEnum + Copy> FunctionEntry<T> {
    pub fn new(function_number: T, function_name: &'static str) -> FunctionEntry<T> {
        FunctionEntry {
            function_number,
            function_name,
        }
    }
    pub fn is_match(&self, function_name: &str) -> bool {
        self.function_name == function_name
    }
    pub fn get_function_number(&self) -> T {
        self.function_number
    }
    pub fn get_function_name(&self) -> &str {
        self.function_name
    }
}   



pub struct FuncDictionary<'a, T: FunctionEnum> {
    table: &'a [FunctionEntry<T>],
    mask: DictionaryMask,
}

// impl<'a> FunctionDictionary<'a, InterpreterFunctions> {
//     pub fn new() -> FunctionDictionary<'a, InterpreterFunctions> {
//         FunctionDictionary {
//             table: &interpreter_function_table::INTERPRETER_FUNCTIONS_TABLE,
//             mask: DictionaryMask::YRSHELL_DICTIONARY_INTERPRETER_FUNCTION,
//         }
//     }
// }
// impl<'a> FunctionDictionary<'a, ShellFunctions> {
//     pub fn new() -> FunctionDictionary<'a, ShellFunctions> {
//         FunctionDictionary {
//             table: &shell_function_table::SHELL_FUNCTIONS_TABLE,
//             mask: DictionaryMask::YRSHELL_DICTIONARY_EXTENSION_FUNCTION,
//         }
//     }
// }

impl<'a, T: FunctionEnum+Copy> Dictionary for FuncDictionary<'a, T> {
    /// get the offset of the firt entry in the dictionary
    fn get_first_entry(&self) -> Option<DictionaryIndex> {
        if self.table.len() == 0 {
            None
        } else {
            Some(DictionaryIndex::FunctionIndex(self.mask, self.table.len() as u16 - 1))
        }
    }
    /// get the offset of the next entry in the dictionary
    fn get_next_entry(&self, index: DictionaryIndex) -> Option<DictionaryIndex> {
        match index {
            DictionaryIndex::FunctionIndex(mask, index) => {
                if index == 0 {
                    None
                } else {
                    Some(DictionaryIndex::FunctionIndex(mask, index - 1))
                }
            }
            _ => None
        }
    }
    /// get the offset of the last entry in the dictionary
    fn get_token(&self, index: DictionaryIndex) -> Option<DictionaryToken> {
        match index {
            DictionaryIndex::FunctionIndex(mask, index) => {
                if index >= self.table.len() as u16 {
                    None
                } else {
                    Some(DictionaryToken::FunctionToken(mask, index))
                }
            }
            _ => None
        }
    }
    fn get_name_address_token(&self, index: DictionaryIndex) -> Option<DictionaryToken> {
        match index {
            DictionaryIndex::FunctionIndex(mask, index) => {
                if index >= self.table.len() as u16 {
                    None
                } else {
                    Some(DictionaryToken::FunctionToken(mask, index))
                }
            }
            _ => None
        }
    }
    fn get_address(&self, index: DictionaryIndex) -> Option<&str> {
        match index {
            DictionaryIndex::FunctionIndex(_mask, index) => {
                if index >= self.table.len() as u16 {
                    None
                } else {
                    Some(self.table[index as usize].function_name)
                }
            }
            _ => None
        }
    }
    fn find(&self, name: &str) -> Option<DictionaryToken> {
        for (index, entry) in self.table.iter().enumerate() {
            if entry.function_name == name {
                return Some(DictionaryToken::CompiledToken(self.mask, index as u16));
            }
        }
        None
    }

    fn find_entry(&self, name: &str) -> Option<DictionaryToken> {
        self.find(name)
    }

    fn get_word(&self, _index: u16) -> Result<u16, ()> {
        panic!("Not implemented");
    }
}