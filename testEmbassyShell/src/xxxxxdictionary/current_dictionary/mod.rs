use super::*;

pub struct CurDictionary<'a>(ComDictionary<'a>);


impl<'a> Dictionary for CurDictionary<'a> {
    /// get the offset of the firt entry in the dictionary
    fn get_first_entry(&self) -> Option<DictionaryIndex> {
        self.0.get_first_entry()
    }
    /// get the offset of the next entry in the dictionary
    fn get_next_entry(&self, index: DictionaryIndex) -> Option<DictionaryIndex> {
        self.0.get_next_entry(index)
    }
    /// get a token with the offset of the first executable entry in the dictionary
    fn get_token(&self, index: DictionaryIndex) -> Option<DictionaryToken> {
        self.0.get_token(index)
    }
    /// get a token with the offset of the name of the entry in the dictionary
    fn get_name_address_token(&self, index: DictionaryIndex) -> Option<DictionaryToken> {
        self.0.get_name_address_token(index)
    }
    /// get a ref to the u16 in the dictionary
    fn get_address(&self, index: DictionaryIndex) -> Option<&str> {
        self.0.get_address(index)
    }

    /// ???
    fn find(&self, name: &str) -> Option<DictionaryToken> {
        self.0.find(name)
    }

    /// get a token that has the index of the entry with this name
    fn find_entry(&self, name: &str) -> Option<DictionaryToken> {
        self.0.find_entry(name)
    }

    fn get_word(&self, _index: u16) -> Result<u16, ()> {
        unimplemented!("Not implemented");
    }
}

impl<'a> CurrentVariableDictionary for CurDictionary<'a> {
    fn new_compile(&self, name: &str) -> bool {
        !unimplemented!("Not implemented");
    }
    fn add_token(&self, token: u16) -> bool {
        !unimplemented!("Not implemented");
    }
    fn set_token(&self, token: u16) -> bool {
        !unimplemented!("Not implemented");
    }
    fn roll_back(&self) {
        !unimplemented!("Not implemented");
    }
    fn new_compile_done(&self) {
        !unimplemented!("Not implemented");
    }

    fn reset(&self) {
        !unimplemented!("Not implemented");
    }
}