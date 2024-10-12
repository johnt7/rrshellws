use super::*;
mod shell_compiled_table;
pub use shell_compiled_table::COMPILED_EXTENSION_DICTIONARY;
mod interpreter_compiled_table;
pub use interpreter_compiled_table::COMPILED_INTERPRETER_DICTIONARY;
use core::str;

// pub struct InterpretedEntry<T: CompiledEnum> {
//     mask: DictionaryMask,
//     function_number: T,     // The identifier used to select the execution code
//     function_name: &'static str, 
// }


// impl<T: InterpretedEnum + Copy> InterpretedEntry<T> {
//     pub fn new(mask: DictionaryMask, function_number: T, function_name: &'static str) -> InterpretedEntry<T> {
//         InterpretedEntry {
//             mask,
//             function_number,
//             function_name,
//         }
//     }
//     pub fn is_match(&self, function_name: &str) -> bool {
//         self.function_name == function_name
//     }
//     pub fn get_function_number(&self) -> T {
//         self.function_number
//     }
//     pub fn get_function_name(&self) -> &str {
//         self.function_name
//     }
// }   



pub struct ComDictionary<'a> {
    table: &'a [u16],
    last_word: u16,
    mask: DictionaryMask,
}

impl<'a> ComDictionary<'a> {
    pub fn new(table : &'a [u16], last_word : u16, mask: DictionaryMask) -> ComDictionary<'a> {
        ComDictionary {
            table,
            last_word,
            mask,
        }
    }
}

impl<'a> ComDictionary<'a> {
    fn find_internal(&self, name: &str) -> Option<DictionaryToken> {
        if self.table.len() == 0 {
            return None;
        }
        let mut posn = self.last_word;
        loop {
            if posn == 0xFFFF {
                //println!("got 0, next is={}", self.table[0]);
                break;
            }
            //println!("name: {}, posn: {} ", name, posn);
            let s = to_str(&self.table[(posn as usize + 1)..]);
            //println!("s={}({}) name={}({})", s, s.len(), name, name.len());
            if s == name {
                return Some(DictionaryToken::CompiledToken(self.mask, posn));
            }
            posn = self.table[posn as usize];
        }
        None
    }
    // fn name_length(&self, posn: u16) -> u16 {
    //     let mut count = 0;
    //     let mut posn = posn;
    //     loop {
    //         if self.table[posn as usize] == 0 {
    //             break;
    //         } else if self.table[posn as usize] & 0xFF == 0 {
    //             count += 1;
    //             break;
    //         }
    //         count += 1;
    //     }
    //     count
    // }
}

impl<'a> Dictionary for ComDictionary<'a> {
    /// get the offset of the firt entry in the dictionary
    fn get_first_entry(&self) -> Option<DictionaryIndex> {
        if self.table.len() == 0 {
            None
        } else {
            Some(DictionaryIndex::FunctionIndex(self.mask, self.last_word))
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
    /// get a token with the offset of the first executable entry in the dictionary
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
    /// get a token with the offset of the name of the entry in the dictionary
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
    /// get a ref to the u16 in the dictionary
    fn get_address(&self, index: DictionaryIndex) -> Option<&str> {
        // if not a compiled token, or index too big, have an error
        // TODO, log this
        let DictionaryIndex::CompiledCode(_mask, index) = index else {
            return None;
        };
        // if index >= self.table.len() as u16 {
        //     return None;
        // };
        // let mut byte_len = 0;
        // let mut count = 0;
        // for _ in index..self.table.len() as u16 {
        //     if (self.table[index as usize]) == 0 {
        //         byte_len += 1;
        //         break;
        //     } else if (self.table[index as usize] & 0xFF) == 0 {
        //         break;
        //     }
        //     byte_len += 2;
        //     count += 1;
        // }
        // let slice = &self.table[index as usize..count];
        // Some(to_u8_slice(slice, byte_len))
        Some(to_str(&self.table[index as usize ..]))
    }

    /// ???
    fn find(&self, name: &str) -> Option<DictionaryToken> {
        self.find_internal(name)
        // let Some(DictionaryToken::CompiledToken(_mask, posn)) = self.find_internal(name) else {
        //     return None;
        // } ;
        // Self::name_length(&self.table[posn as usize..])
    }

    /// get a token that has the index of the entry with this name
    fn find_entry(&self, name: &str) -> Option<DictionaryToken> {
        self.find_internal(name)
    }

    fn get_word(&self, _index: u16) -> Result<u16, ()> {
        panic!("Not implemented");
    }
}

fn to_str(slice: &[u16]) -> &str {
    let mut byte_len = 0;
    let mut count = 0;
    for i in 0..slice.len() as u16 {
        //println!("slice[i]={:x}", slice[i as usize]);
        if (slice[i as usize]) & 0x00FF == 0 {
            break;
        } else if (slice[i as usize] & 0xFF00) == 0 {
            byte_len += 1;
            break;
        }
        byte_len += 2;
        count += 1;
    }
    let slice = &slice[0..count];
    to_u8_slice(slice, byte_len)
}

fn to_u8_slice(slice: &[u16], byte_len: usize) -> &str {
    //println!("byte_len: {}, bytes={:?}", byte_len, slice);
    if slice.len() ==0 {
        return "";
    }
    unsafe {
        let s = core::slice::from_raw_parts(
            slice.as_ptr().cast::<u8>(),
            byte_len
        );
//        println!("bytes={:?}", s);
        str::from_utf8(s).unwrap()
    }
}
