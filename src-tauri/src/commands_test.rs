#[cfg(test)]
mod tests {
    use super::*;
    use crate::sled_manager::{SledManager, KeyValue, ValueType};

    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // 创建一个测试用的SledManager实例
    fn create_test_manager() -> (SledManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let manager = SledManager::new();
        (manager, temp_dir)
    }

    // 创建测试连接
    fn create_test_connection(manager: &SledManager, name: &str, temp_dir: &TempDir) -> String {
        let db_path = temp_dir.path().join(format!("{}.db", name));
        let result = manager.add_connection(name.to_string(), db_path);
        assert!(result.is_ok());
        result.unwrap()
    }

    #[test]
    fn test_create_connection() {
        let (manager, temp_dir) = create_test_manager();
        let db_path = temp_dir.path().join("test.db");
        
        let request = CreateConnectionRequest {
            name: "test_connection".to_string(),
            path: db_path.to_string_lossy().to_string(),
        };
        
        // 直接调用manager的方法，而不是通过State
        let result = manager.add_connection(request.name, PathBuf::from(request.path));
        assert!(result.is_ok());
        
        let connection_id = result.unwrap();
        assert!(!connection_id.is_empty());
        
        // 验证连接是否已添加
        let connections = manager.get_connections();
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].name, "test_connection");
    }

    #[test]
    fn test_remove_connection() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "test_remove", &temp_dir);
        
        // 确认连接存在
        let connections = manager.get_connections();
        assert_eq!(connections.len(), 1);
        
        // 删除连接
        let result = manager.remove_connection(&connection_id);
        assert!(result.is_ok());
        
        // 确认连接已删除
        let connections = manager.get_connections();
        assert_eq!(connections.len(), 0);
    }

    #[test]
    fn test_get_connections() {
        let (manager, temp_dir) = create_test_manager();
        
        // 初始状态应该没有连接
        let connections = manager.get_connections();
        assert_eq!(connections.len(), 0);
        
        // 添加两个连接
        let conn1_id = create_test_connection(&manager, "conn1", &temp_dir);
        let conn2_id = create_test_connection(&manager, "conn2", &temp_dir);
        
        // 验证连接列表
        let connections = manager.get_connections();
        assert_eq!(connections.len(), 2);
        
        // 验证连接详情
        let conn1 = manager.get_connection(&conn1_id);
        assert!(conn1.is_some());
        assert_eq!(conn1.unwrap().name, "conn1");
        
        let conn2 = manager.get_connection(&conn2_id);
        assert!(conn2.is_some());
        assert_eq!(conn2.unwrap().name, "conn2");
    }

    #[test]
    fn test_create_and_remove_tree() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "tree_test", &temp_dir);
        
        // 创建树
        let result = manager.open_tree(&connection_id, "test_tree");
        assert!(result.is_ok());
        
        // 验证树存在
        let trees = manager.get_trees(&connection_id).unwrap();
        assert!(trees.contains(&"test_tree".to_string()));
        
        // 删除树 - 需要通过数据库实例来删除
        let db = manager.get_database(&connection_id).unwrap();
        let result = db.drop_tree("test_tree");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_and_set() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "kv_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 设置键值
        let result = tree.insert(b"test_key", b"test_value");
        assert!(result.is_ok());
        
        // 获取键值
        let result = tree.get(b"test_key");
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_vec(), b"test_value".to_vec());
    }

    #[test]
    fn test_remove() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "remove_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 先设置一个键值
        let result = tree.insert(b"key_to_remove", b"value_to_remove");
        assert!(result.is_ok());
        
        // 删除键值
        let result = tree.remove(b"key_to_remove");
        assert!(result.is_ok());
        
        // 验证键值已删除
        let result = tree.get(b"key_to_remove");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_range_query() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "range_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 添加测试数据
        for i in 0..10 {
            let key = format!("key_{:02}", i);
            let value = format!("value_{}", i);
            let result = tree.insert(key.as_bytes(), value.as_bytes());
            assert!(result.is_ok());
        }
        
        // 范围查询
        let from_key = b"key_03".to_vec();
        let to_key = b"key_07".to_vec();
        
        let mut results = Vec::new();
        for item in tree.range(from_key..to_key).take(5) {
            let (key, value) = item.unwrap();
            results.push((key.to_vec(), value.to_vec()));
        }
        
        assert_eq!(results.len(), 4); // key_03, key_04, key_05, key_06
        assert_eq!(results[0].0, b"key_03".to_vec());
        assert_eq!(results[3].0, b"key_06".to_vec());
    }

    #[test]
    fn test_prefix_query() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "prefix_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 添加测试数据
        for i in 0..5 {
            let key = format!("user_{}", i);
            let value = format!("user_data_{}", i);
            let result = tree.insert(key.as_bytes(), value.as_bytes());
            assert!(result.is_ok());
        }
        
        // 添加其他数据
        for i in 0..3 {
            let key = format!("admin_{}", i);
            let value = format!("admin_data_{}", i);
            let result = tree.insert(key.as_bytes(), value.as_bytes());
            assert!(result.is_ok());
        }
        
        // 前缀查询
        let prefix = b"user_".to_vec();
        let mut results = Vec::new();
        
        for item in tree.scan_prefix(prefix).take(10) {
            let (key, value) = item.unwrap();
            results.push((key.to_vec(), value.to_vec()));
        }
        
        assert_eq!(results.len(), 5); // 所有user_开头的键
        
        for (key, _) in &results {
            assert!(key.starts_with(b"user_"));
        }
    }

    #[test]
    fn test_import_data() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "import_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 准备导入数据
        let import_data = vec![
            KeyValue {
                key: b"import_key_1".to_vec(),
                value: b"import_value_1".to_vec(),
                value_type: ValueType::String,
            },
            KeyValue {
                key: b"import_key_2".to_vec(),
                value: b"import_value_2".to_vec(),
                value_type: ValueType::String,
            },
        ];
        
        // 导入数据
        let mut _count = 0;
        for kv in import_data {
            let result = tree.insert(kv.key.as_slice(), kv.value.as_slice());
            assert!(result.is_ok());
            _count += 1;
        }
        
        // 验证导入的数据
        assert_eq!(_count, 2); // 导入了2条记录
        
        // 验证导入的数据
        let result = tree.get(b"import_key_1");
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_vec(), b"import_value_1".to_vec());
    }

    #[test]
    fn test_export_data() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "export_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 添加测试数据
        let result = tree.insert(b"export_key_1", b"export_value_1");
        assert!(result.is_ok());
        
        let result = tree.insert(b"export_key_2", b"export_value_2");
        assert!(result.is_ok());
        
        // 导出数据
        let export_path = temp_dir.path().join("export.json");
        
        // 收集所有键值对
        let mut export_data = Vec::new();
        for item in tree.iter() {
            let (key, value) = item.unwrap();
            let kv = KeyValue {
                key: key.to_vec(),
                value: value.to_vec(),
                value_type: SledManager::detect_value_type(&value),
            };
            export_data.push(kv);
        }
        
        // 写入JSON文件
        let json_content = serde_json::to_string_pretty(&export_data).unwrap();
        fs::write(&export_path, json_content).unwrap();
        
        // 验证文件是否存在
        assert!(export_path.exists());
        
        // 验证文件内容
        let content = fs::read_to_string(&export_path).unwrap();
        // 由于key和value是Vec<u8>，它们会被序列化为格式化的字节数组
        assert!(content.contains("\"key\": [\n      101,\n      120,\n      112,\n      111,\n      114,\n      116,\n      95,\n      107,\n      101,\n      121,\n      95,\n      49\n    ]")); // "export_key_1"的字节数组
        assert!(content.contains("\"key\": [\n      101,\n      120,\n      112,\n      111,\n      114,\n      116,\n      95,\n      107,\n      101,\n      121,\n      95,\n      50\n    ]")); // "export_key_2"的字节数组
    }

    #[test]
    fn test_import_from_path() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "import_path_test", &temp_dir);
        
        // 创建测试JSON文件
        let json_content = r#"
[
  {"key": [112, 97, 116, 104, 95, 107, 101, 121, 95, 49], "value": [112, 97, 116, 104, 95, 118, 97, 108, 117, 101, 95, 49], "value_type": "String"},
  {"key": [112, 97, 116, 104, 95, 107, 101, 121, 95, 50], "value": [112, 97, 116, 104, 95, 118, 97, 108, 117, 101, 95, 50], "value_type": "String"}
]
"#;
        
        let import_path = temp_dir.path().join("import.json");
        fs::write(&import_path, json_content).unwrap();
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 从文件导入
        let content = fs::read_to_string(&import_path).unwrap();
        let import_data: Vec<KeyValue> = serde_json::from_str(&content).unwrap();
        
        let mut _count = 0;
        for kv in import_data {
            let result = tree.insert(kv.key.as_slice(), kv.value.as_slice());
            assert!(result.is_ok());
            _count += 1;
        }
        
        // 验证导入的数据
        let result = tree.get(b"path_key_1");
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_vec(), b"path_value_1".to_vec());
        
        // 验证第二条数据
        let result = tree.get(b"path_key_2");
        assert!(result.is_ok());
        
        let value = result.unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_vec(), b"path_value_2".to_vec());
    }

    #[test]
    fn test_get_stats() {
        let (manager, temp_dir) = create_test_manager();
        let connection_id = create_test_connection(&manager, "stats_test", &temp_dir);
        
        // 获取数据库实例
        let db = manager.get_database(&connection_id).unwrap();
        let tree = db.open_tree("default").unwrap();
        
        // 添加一些数据
        for i in 0..5 {
            let key = format!("stats_key_{}", i);
            let value = format!("stats_value_{}", i);
            let result = tree.insert(key.as_bytes(), value.as_bytes());
            assert!(result.is_ok());
        }
        
        // 获取统计信息
        let result = manager.get_stats(&connection_id);
        assert!(result.is_ok());
        
        let _stats = result.unwrap();
        // 验证统计信息包含预期的字段
        // 注意：具体字段取决于SledManager的DbStats结构
    }
}