use crate::sled_manager::{SledManager, KeyValue, QueryResult, RangeQuery, PrefixQuery, ValueType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;
use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use serde_json;
use csv;
use quick_xml::se::to_string as to_xml_string;
use yaml_rust2::yaml::{Hash, Array, Yaml};
use yaml_rust2::YamlLoader;
use yaml_rust2::YamlEmitter;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeQueryRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub query: RangeQuery,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrefixQueryRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub query: PrefixQuery,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub data: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub format: String, // "json", "csv", "xml", "yaml"
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportFromPathRequest {
    pub connection_id: String,
    pub tree_name: Option<String>,
    pub file_path: String,
    pub format: String, // "json", "csv", "xml", "yaml"
}

#[tauri::command]
pub fn create_connection(
    request: CreateConnectionRequest,
    manager: State<'_, SledManager>,
) -> Result<String, String> {
    let path = PathBuf::from(request.path);
    manager
        .add_connection(request.name, path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_connection(
    connection_id: String,
    manager: State<'_, SledManager>,
) -> Result<(), String> {
    manager
        .remove_connection(&connection_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_connections(
    manager: State<'_, SledManager>,
) -> Result<Vec<crate::sled_manager::ConnectionInfo>, String> {
    Ok(manager.get_connections())
}

#[tauri::command]
pub fn get_connection(
    connection_id: String,
    manager: State<'_, SledManager>,
) -> Result<Option<crate::sled_manager::ConnectionInfo>, String> {
    Ok(manager.get_connection(&connection_id))
}

#[tauri::command]
pub fn get_trees(
    connection_id: String,
    manager: State<'_, SledManager>,
) -> Result<Vec<String>, String> {
    manager
        .get_trees(&connection_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_stats(
    connection_id: String,
    manager: State<'_, SledManager>,
) -> Result<crate::sled_manager::DbStats, String> {
    manager
        .get_stats(&connection_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get(
    request: GetRequest,
    manager: State<'_, SledManager>,
) -> Result<Option<KeyValue>, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let result = tree.get(&request.key).map_err(|e| e.to_string())?;
    
    match result {
        Some(value) => {
            let value_type = SledManager::detect_value_type(&value);
            Ok(Some(KeyValue {
                key: request.key,
                value: value.to_vec(),
                value_type,
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub fn set(
    request: SetRequest,
    manager: State<'_, SledManager>,
) -> Result<Option<Vec<u8>>, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let result = tree.insert(&request.key, &*request.value).map_err(|e| e.to_string())?;
    
    Ok(result.map(|v| v.to_vec()))
}

#[tauri::command]
pub fn remove(
    request: RemoveRequest,
    manager: State<'_, SledManager>,
) -> Result<Option<Vec<u8>>, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let result = tree.remove(&request.key).map_err(|e| e.to_string())?;
    
    Ok(result.map(|v| v.to_vec()))
}

#[tauri::command]
pub fn range_query(
    request: RangeQueryRequest,
    manager: State<'_, SledManager>,
) -> Result<QueryResult, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let iter = match (request.query.from.as_deref(), request.query.to.as_deref()) {
        (Some(from), Some(to)) => tree.range(from..to),
        (Some(from), None) => tree.range(from..),
        (None, Some(to)) => tree.range(..to),
        (None, None) => tree.range::<&[u8], std::ops::RangeFull>(..),
    };
    
    let mut entries = Vec::new();
    let mut count = 0;
    let mut has_more = false;
    
    let iter: Box<dyn Iterator<Item = Result<(_, _), _>>> = if request.query.reverse {
        Box::new(iter.rev())
    } else {
        Box::new(iter)
    };
    
    for item in iter {
        let (key, value) = item.map_err(|e| e.to_string())?;
        
        if let Some(limit) = request.query.limit {
            if count >= limit {
                has_more = true;
                break;
            }
        }
        
        let value_type = SledManager::detect_value_type(&value);
        entries.push(KeyValue {
            key: key.to_vec(),
            value: value.to_vec(),
            value_type,
        });
        count += 1;
    }
    
    let total_count = tree.len();
    
    Ok(QueryResult {
        entries,
        total_count,
        has_more,
    })
}

#[tauri::command]
pub fn prefix_query(
    request: PrefixQueryRequest,
    manager: State<'_, SledManager>,
) -> Result<QueryResult, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let prefix = request.query.prefix;
    let mut entries = Vec::new();
    let mut count = 0;
    let mut has_more = false;
    
    for item in tree.scan_prefix(&prefix) {
        let (key, value) = item.map_err(|e| e.to_string())?;
        
        if let Some(limit) = request.query.limit {
            if count >= limit {
                has_more = true;
                break;
            }
        }
        
        let value_type = SledManager::detect_value_type(&value);
        entries.push(KeyValue {
            key: key.to_vec(),
            value: value.to_vec(),
            value_type,
        });
        count += 1;
    }
    
    let total_count = tree.len();
    
    Ok(QueryResult {
        entries,
        total_count,
        has_more,
    })
}

#[tauri::command]
pub fn import_data(
    request: ImportRequest,
    manager: State<'_, SledManager>,
) -> Result<usize, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let mut count = 0;
    for kv in request.data {
        tree.insert(&kv.key, &*kv.value).map_err(|e| e.to_string())?;
        count += 1;
    }
    
    Ok(count)
}

#[tauri::command]
pub fn export_data(
    request: ExportRequest,
    manager: State<'_, SledManager>,
) -> Result<String, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let mut entries = Vec::new();
    
    for item in tree.iter() {
        let (key, value) = item.map_err(|e| e.to_string())?;
        let value_type = SledManager::detect_value_type(&value);
        entries.push(KeyValue {
            key: key.to_vec(),
            value: value.to_vec(),
            value_type,
        });
    }
    
    // 根据格式导出数据
    match request.format.as_str() {
        "json" => {
            let json_content = serde_json::to_string_pretty(&entries)
                .map_err(|e| e.to_string())?;
            let mut file = File::create(&request.file_path)
                .map_err(|e| e.to_string())?;
            file.write_all(json_content.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        "csv" => {
            let mut file = File::create(&request.file_path)
                .map_err(|e| e.to_string())?;
            let mut wtr = csv::Writer::from_writer(file);
            
            // 写入CSV头部
            wtr.write_record(&["key", "value", "value_type"])
                .map_err(|e| e.to_string())?;
            
            // 写入数据行
            for entry in &entries {
                let key_str = String::from_utf8_lossy(&entry.key);
                let value_str = String::from_utf8_lossy(&entry.value);
                let value_type_str = format!("{:?}", entry.value_type);
                
                wtr.write_record(&[&key_str[..], &value_str[..], &value_type_str])
                    .map_err(|e| e.to_string())?;
            }
            
            wtr.flush().map_err(|e| e.to_string())?;
        }
        "xml" => {
            // 为XML导出创建一个包装结构
            #[derive(Serialize)]
            struct XmlExport {
                #[serde(rename = "entry")]
                entries: Vec<XmlEntry>,
            }
            
            #[derive(Serialize)]
            struct XmlEntry {
                #[serde(rename = "$value")]
                key: String,
                value: String,
                value_type: String,
            }
            
            let xml_entries: Vec<XmlEntry> = entries.iter().map(|entry| {
                let key_str = String::from_utf8_lossy(&entry.key);
                let value_str = String::from_utf8_lossy(&entry.value);
                let value_type_str = format!("{:?}", entry.value_type);
                
                XmlEntry {
                    key: key_str.to_string(),
                    value: value_str.to_string(),
                    value_type: value_type_str,
                }
            }).collect();
            
            let xml_export = XmlExport { entries: xml_entries };
            let xml_content = to_xml_string(&xml_export)
                .map_err(|e| e.to_string())?;
            
            let mut file = File::create(&request.file_path)
                .map_err(|e| e.to_string())?;
            file.write_all(xml_content.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        "yaml" => {
            // 将entries转换为YAML格式
            let mut yaml_array = Array::new();
            
            for entry in &entries {
                let mut yaml_hash = Hash::new();
                
                let key_str = String::from_utf8_lossy(&entry.key);
                let value_str = String::from_utf8_lossy(&entry.value);
                let value_type_str = format!("{:?}", entry.value_type);
                
                yaml_hash.insert(
                    Yaml::String("key".to_string()), 
                    Yaml::String(key_str.to_string())
                );
                yaml_hash.insert(
                    Yaml::String("value".to_string()), 
                    Yaml::String(value_str.to_string())
                );
                yaml_hash.insert(
                    Yaml::String("value_type".to_string()), 
                    Yaml::String(value_type_str)
                );
                
                yaml_array.push(Yaml::Hash(yaml_hash));
            }
            
            let yaml_content = Yaml::Array(yaml_array);
            let mut yaml_str = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut yaml_str);
                emitter.dump(&yaml_content)
                    .map_err(|e| e.to_string())?;
            }
            
            let mut file = File::create(&request.file_path)
                .map_err(|e| e.to_string())?;
            file.write_all(yaml_str.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        _ => {
            return Err("不支持的导出格式".to_string());
        }
    }
    
    Ok(format!("成功导出 {} 条记录到 {}", entries.len(), request.file_path))
}

#[tauri::command]
pub fn import_from_path(
    request: ImportFromPathRequest,
    manager: State<'_, SledManager>,
) -> Result<String, String> {
    let tree = match request.tree_name {
        Some(name) => manager.open_tree(&request.connection_id, &name).map_err(|e| e.to_string())?,
        None => {
            let db = manager.get_database(&request.connection_id).map_err(|e| e.to_string())?;
            // Use the default tree
            Arc::new(db.open_tree("default").map_err(|e| e.to_string())?)
        }
    };
    
    let mut file = File::open(&request.file_path)
        .map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    
    let mut count = 0;
    
    match request.format.as_str() {
        "json" => {
            let entries: Vec<KeyValue> = serde_json::from_str(&contents)
                .map_err(|e| e.to_string())?;
            
            for entry in entries {
                tree.insert(&entry.key, &*entry.value)
                    .map_err(|e| e.to_string())?;
                count += 1;
            }
        }
        "csv" => {
            let mut rdr = csv::Reader::from_reader(contents.as_bytes());
            
            for result in rdr.records() {
                let record = result.map_err(|e| e.to_string())?;
                
                if record.len() >= 2 {
                    let key = record.get(0).unwrap_or("").as_bytes().to_vec();
                    let value = record.get(1).unwrap_or("").as_bytes().to_vec();
                    
                    tree.insert(&key, &*value)
                        .map_err(|e| e.to_string())?;
                    count += 1;
                }
            }
        }
        "yaml" => {
            // 使用yaml-rust2库处理YAML格式
            let yaml_docs = YamlLoader::load_from_str(&contents)
                .map_err(|e| e.to_string())?;
            
            if yaml_docs.is_empty() {
                return Err("YAML文档为空".to_string());
            }
            
            let yaml_doc = &yaml_docs[0];
            
            // 处理YAML文档中的键值对
            match yaml_doc {
                Yaml::Hash(hash) => {
                    for (key, value) in hash {
                        if let (Some(key_str), Some(value_str)) = (
                            key.as_str(), 
                            value.as_str()
                        ) {
                            let key_bytes = key_str.as_bytes().to_vec();
                            let value_bytes = value_str.as_bytes().to_vec();
                            
                            tree.insert(&key_bytes, &*value_bytes)
                                .map_err(|e| e.to_string())?;
                            count += 1;
                        }
                    }
                }
                Yaml::Array(array) => {
                    for item in array {
                        if let Yaml::Hash(hash) = item {
                            for (key, value) in hash {
                                if let (Some(key_str), Some(value_str)) = (
                                    key.as_str(), 
                                    value.as_str()
                                ) {
                                    let key_bytes = key_str.as_bytes().to_vec();
                                    let value_bytes = value_str.as_bytes().to_vec();
                                    
                                    tree.insert(&key_bytes, &*value_bytes)
                                        .map_err(|e| e.to_string())?;
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                _ => {
                    return Err("不支持的YAML格式，需要对象或对象数组".to_string());
                }
            }
        }
        _ => {
            return Err("不支持的导入格式".to_string());
        }
    }
    
    Ok(format!("成功导入 {} 条记录", count))
}

#[tauri::command]
pub fn create_tree(
    connection_id: String,
    tree_name: String,
    manager: State<'_, SledManager>,
) -> Result<(), String> {
    // 在Sled中，创建树实际上是通过open_tree完成的
    // 如果树不存在，open_tree会自动创建它
    manager
        .open_tree(&connection_id, &tree_name)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn remove_tree(
    connection_id: String,
    tree_name: String,
    manager: State<'_, SledManager>,
) -> Result<(), String> {
    let db = manager.get_database(&connection_id).map_err(|e| e.to_string())?;
    
    // 从数据库中删除指定的树
    db.drop_tree(&tree_name).map_err(|e| e.to_string())?;
    
    Ok(())
 }

// 包含测试模块
#[cfg(test)]
include!("commands_test.rs");