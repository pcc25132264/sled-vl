use sled::{Db, Tree, IVec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub id: String,
    pub name: String,
    #[serde(serialize_with = "serialize_pathbuf")]
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

// Helper function to serialize PathBuf as string
fn serialize_pathbuf<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let path_str = path.to_string_lossy().to_string();
    serializer.serialize_str(&path_str)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    Json,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub entries: Vec<KeyValue>,
    pub total_count: usize,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbStats {
    pub size_on_disk: u64,
    pub key_count: usize,
    pub tree_count: usize,
    pub last_modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub id: String,
    pub is_active: bool,
    pub operations_count: usize,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeQuery {
    pub from: Option<Vec<u8>>,
    pub to: Option<Vec<u8>>,
    pub limit: Option<usize>,
    pub reverse: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixQuery {
    pub prefix: Vec<u8>,
    pub limit: Option<usize>,
}

pub type DbManager = Arc<Mutex<HashMap<String, Arc<Db>>>>;

pub struct SledManager {
    pub connections: Arc<Mutex<HashMap<String, ConnectionInfo>>>,
    pub databases: DbManager,
}

impl SledManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            databases: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_connection(&self, name: String, path: PathBuf) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let connection_info = ConnectionInfo {
            id: id.clone(),
            name,
            path: path.clone(),
            created_at: now,
            last_accessed: now,
        };

        // Open the database
        let db = sled::open(&path)?;
        
        // Store connection info
        self.connections.lock().unwrap().insert(id.clone(), connection_info);
        
        // Store database instance
        self.databases.lock().unwrap().insert(id.clone(), Arc::new(db));
        
        Ok(id)
    }

    pub fn remove_connection(&self, id: &str) -> Result<()> {
        self.connections.lock().unwrap().remove(id);
        self.databases.lock().unwrap().remove(id);
        
        Ok(())
    }

    pub fn get_connections(&self) -> Vec<ConnectionInfo> {
        self.connections.lock().unwrap().values().cloned().collect()
    }

    pub fn get_connection(&self, id: &str) -> Option<ConnectionInfo> {
        self.connections.lock().unwrap().get(id).cloned()
    }

    pub fn get_database(&self, id: &str) -> Result<Arc<Db>> {
        self.databases
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| anyhow!("Database connection not found: {}", id))
    }

    pub fn update_last_accessed(&self, id: &str) -> Result<()> {
        if let Some(connection) = self.connections.lock().unwrap().get_mut(id) {
            connection.last_accessed = Utc::now();
        }
        Ok(())
    }

    pub fn get_trees(&self, connection_id: &str) -> Result<Vec<String>> {
        let db = self.get_database(connection_id)?;
        let trees: Vec<String> = db.tree_names()
            .into_iter()
            .map(|name| String::from_utf8_lossy(&name).to_string())
            .collect();
        Ok(trees)
    }

    pub fn open_tree(&self, connection_id: &str, tree_name: &str) -> Result<Arc<Tree>> {
        let db = self.get_database(connection_id)?;
        let tree = db.open_tree(tree_name)?;
        Ok(Arc::new(tree))
    }

    pub fn get_stats(&self, connection_id: &str) -> Result<DbStats> {
        let db = self.get_database(connection_id)?;
        
        // Get size on disk
        let size_on_disk = db.size_on_disk()?;
        
        // Get key count from default tree
        let key_count = db.len();
        
        // Get tree count
        let tree_count = db.tree_names().len();
        
        // Get last modified time (approximation)
        let last_modified = Utc::now();
        
        Ok(DbStats {
            size_on_disk,
            key_count,
            tree_count,
            last_modified,
        })
    }

    pub fn detect_value_type(value: &[u8]) -> ValueType {
        // Try to detect the value type
        if let Ok(s) = std::str::from_utf8(value) {
            // Check if it's JSON
            if s.trim_start().starts_with('{') || s.trim_start().starts_with('[') {
                return ValueType::Json;
            }
            
            // Check if it's a boolean
            if s == "true" || s == "false" {
                return ValueType::Boolean;
            }
            
            // Check if it's a number
            if s.parse::<f64>().is_ok() {
                return ValueType::Number;
            }
            
            // Otherwise, treat as string
            return ValueType::String;
        }
        
        // If not valid UTF-8, treat as binary
        ValueType::Binary
    }
}

pub fn init_sled_manager() -> SledManager {
    SledManager::new()
}