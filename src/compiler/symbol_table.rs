use std::collections::HashMap;

/// A symbol table
#[derive(Debug, Clone)]
pub struct SymbolTable<K, V> {
    /// The parent symbol table
    parent: Option<Box<SymbolTable<K, V>>>,
    /// The symbols
    symbols: HashMap<K, V>,
}