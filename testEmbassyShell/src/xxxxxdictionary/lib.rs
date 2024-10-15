pub mod function_dictionary;
use function_dictionary::FuncDictionary;
pub mod compiled_dictionary;
use compiled_dictionary::ComDictionary;
pub mod current_dictionary;
use current_dictionary::CurDictionary;
//mod interpreter_function_table;
//mod interpreter_functions;
//pub use interpreter_functions::InterpreterFunctions;
//pub use shell_functions::ShellFunctions;

pub enum DictionaryIndex {
    FunctionIndex(DictionaryMask, u16),     // index of a word in a function dictionary
    CurrentIndex(u16),                      // index of a word in the current dictionary
    CompiledCode(DictionaryMask, u16),      // index of a word's executable code in a compiled dictionary
    Current(u16),
    Relative(u16),
}
#[derive(Debug, PartialEq)]
enum DictionaryToken {
    FunctionToken(DictionaryMask, u16), // Token for a word in a function dictionary
    CompiledToken(DictionaryMask, u16), // Token for a word in a compiled dictionary
    Current(u16),                       // Token for a word in the current dictionary
    Relative(u16),                      // Token for a word on the stack
}

//impl FunctionToken {}

/// mask is the identity of the dictionary, mapped onto the  high bits of the token
/// Entries are indexes into the dictionary.  Ued only for iterating through the dictionaries
/// Tokens are the executable code
pub trait Dictionary {
    // TODO, these will need to be revisited when I start on the execution
    // I think we can get away without this
    // /// Mask is used to distinguish between the dictionary and the current dictionary
    // fn get_mask(&self) -> DictionaryMask;

    /// get the index of the firt entry in the dictionary
    fn get_first_entry(&self) -> Option<DictionaryIndex>;
    /// get the offset of the next entry in the dictionary
    fn get_next_entry(&self, index: DictionaryIndex) -> Option<DictionaryIndex>;
    
    /// get an executable token from the dictionary, at the index
    fn get_token(&self, index: DictionaryIndex) ->  Option<DictionaryToken>;
    /// get an excutable  token from the dictionary, with this name
    fn get_name_address_token(&self, index: DictionaryIndex) -> Option<DictionaryToken>;

    /// gets a reference to the name of the entry
    fn get_address(&self, index: DictionaryIndex) -> Option<&str>;
    /// get the execution token of the entry with this name
    fn find(&self, name: &str) -> Option<DictionaryToken>;
    /// get a token of the entry with this name that points to the nam
    fn find_entry(&self, name: &str) -> Option<DictionaryToken>;
    /// returns the word at the index in the dictionary, only used for compiled dictionary
    fn get_word(&self, index: u16) -> Result<u16, ()>;

    // length of the name, in bytes
    fn name_length(name: &[u16]) -> u16 {
        // let len = name.len() as u16 + 1; 
        // len/2 + (len & 1) ;

        let mut count = 0;
        loop {
            if name[count as usize] == 0 {
                count *= 2;
                break;
            } else if name[count as usize] & 0xFF == 0 {
                count = count*2 + 1;
                break;
            }
            count += 1;
        }
        count
    }

}

pub trait ExecutableEnum {}
pub trait FunctionEnum : ExecutableEnum{}
pub trait CompiledEnum : ExecutableEnum{}

pub trait CompiledDictionary: Dictionary {
    fn get_size(&self) -> u16;
    fn get_word_end() -> u16;
}

pub trait CurrentVariableDictionary: Dictionary {
    fn new_compile(&self, name: &str) -> bool;
    fn add_token(&self, token: u16) -> bool;
    fn set_token(&self, token: u16) -> bool;
    fn roll_back(&self);
    fn new_compile_done(&self);

     fn reset(&self);
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DictionaryMask {
    YRSHELL_DICTIONARY_CURRENT = 0,
    YRSHELL_DICTIONARY_EXTENSION_COMPILED,
    YRSHELL_DICTIONARY_EXTENSION_FUNCTION,
    YRSHELL_DICTIONARY_INTERPRETER_COMPILED,
    YRSHELL_DICTIONARY_COMMON_FUNCTION,
    YRSHELL_DICTIONARY_INTERPRETER_FUNCTION,
    YRSHELL_DICTIONARY_RELATIVE
}


/// DictionaryEntry - points to the parts of a distionary entry
#[derive(Debug, Copy, Clone)]
struct DictionaryEntry {
    mask: u16,
    token: u16,
    name_address: u16,
    code: u16,
}


