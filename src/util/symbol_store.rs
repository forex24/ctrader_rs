use crate::util::symbol_info::SpotwareSymbolInfo;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolStore {
    symbol_to_id: HashMap<String, i64>,
    id_to_symbol: HashMap<i64, String>,
    symbol_to_info: HashMap<String, SpotwareSymbolInfo>,
}

impl SymbolStore {
    pub(crate) fn new() -> Self {
        Self {
            symbol_to_id: HashMap::new(),
            id_to_symbol: HashMap::new(),
            symbol_to_info: HashMap::new(),
        }
    }

    pub fn from_symbol_infos(&mut self, infos: &Vec<SpotwareSymbolInfo>) {
        for info in infos {
            let symbol = &info.name;
            let id = info.info.symbol_id;

            self.symbol_to_id.insert(symbol.to_string(), id);
            self.id_to_symbol.insert(id, symbol.to_string());
            self.symbol_to_info.insert(symbol.to_string(), info.clone());
        }
    }

    pub fn build_from_symbol_infos(infos: Vec<SpotwareSymbolInfo>) -> Self {
        let mut result = Self::new();
        result.from_symbol_infos(&infos);
        result
    }

    pub fn get_ids(&self) -> Vec<i64> {
        self.id_to_symbol.keys().copied().collect::<Vec<_>>()
    }

    pub fn get_name_by_id(&self, id: i64) -> Option<&String> {
        self.id_to_symbol.get(&id)
    }

    pub fn get_id_by_name(&self, symbol_name: &str) -> Option<i64> {
        self.symbol_to_id.get(symbol_name).copied()
    }

    pub fn get_info_by_name<'a>(&'a self, symbol_name: &str) -> Option<&'a SpotwareSymbolInfo> {
        self.symbol_to_info.get(symbol_name)
    }

    pub fn get_info_by_id(&self, id: i64) -> Option<&SpotwareSymbolInfo> {
        let name = self.get_name_by_id(id);
        if let Some(symbol_name) = name {
            return self.symbol_to_info.get(symbol_name);
        }
        None
    }
}
