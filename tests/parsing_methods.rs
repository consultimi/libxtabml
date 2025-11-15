use libxtabml::XtabMLParser;
use std::fs;
use std::path::Path;

/// Helper function to get the path to the example file
fn example_file_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("resources")
        .join("example.xte")
        .to_string_lossy()
        .to_string()
}

#[test]
fn test_parse_file_method() {
    let result = XtabMLParser::parse_file(&example_file_path());
    assert!(result.is_ok(), "parse_file should succeed");
    
    let xtab = result.unwrap();
    assert_eq!(xtab.version, "Observation");
    assert!(!xtab.tables.is_empty());
}

#[test]
fn test_parse_file_nonexistent() {
    let result = XtabMLParser::parse_file("nonexistent_file.xte");
    assert!(result.is_err(), "parse_file should fail for nonexistent file");
    
    let error = result.unwrap_err();
    assert!(matches!(error, libxtabml::XtabMLError::Io(_)));
}

#[test]
fn test_parse_str_method() {
    let file_content = fs::read_to_string(&example_file_path())
        .expect("Should be able to read example file");
    
    let result = XtabMLParser::parse_str(&file_content);
    assert!(result.is_ok(), "parse_str should succeed");
    
    let xtab = result.unwrap();
    assert_eq!(xtab.version, "Observation");
    assert!(!xtab.tables.is_empty());
}

#[test]
fn test_parse_str_empty() {
    let result = XtabMLParser::parse_str("");
    // Note: Current parser implementation doesn't fail on empty string
    // This test documents the current behavior
    assert!(result.is_ok(), "parse_str currently succeeds for empty string");
}

#[test]
fn test_parse_str_invalid_xml() {
    let invalid_xml = r#"<xtab><invalid></xtab>"#;
    let result = XtabMLParser::parse_str(invalid_xml);
    assert!(result.is_err(), "parse_str should fail for invalid XML");
    
    let error = result.unwrap_err();
    assert!(matches!(error, libxtabml::XtabMLError::XmlParse(_)));
}

#[test]
fn test_parse_bytes_method() {
    let file_content = fs::read(&example_file_path())
        .expect("Should be able to read example file as bytes");
    
    let result = XtabMLParser::parse_bytes(&file_content);
    assert!(result.is_ok(), "parse_bytes should succeed");
    
    let xtab = result.unwrap();
    assert_eq!(xtab.version, "Observation");
    assert!(!xtab.tables.is_empty());
}

#[test]
fn test_parse_bytes_empty() {
    let empty_bytes: &[u8] = &[];
    let result = XtabMLParser::parse_bytes(empty_bytes);
    // Note: Current parser implementation doesn't fail on empty bytes
    // This test documents the current behavior
    assert!(result.is_ok(), "parse_bytes currently succeeds for empty bytes");
}

#[test]
fn test_parsing_methods_consistency() {
    // Read the file content once
    let file_content_str = fs::read_to_string(&example_file_path())
        .expect("Should be able to read example file");
    let file_content_bytes = fs::read(&example_file_path())
        .expect("Should be able to read example file as bytes");
    
    // Parse using all three methods
    let result_file = XtabMLParser::parse_file(&example_file_path());
    let result_str = XtabMLParser::parse_str(&file_content_str);
    let result_bytes = XtabMLParser::parse_bytes(&file_content_bytes);
    
    // All should succeed
    assert!(result_file.is_ok(), "parse_file should succeed");
    assert!(result_str.is_ok(), "parse_str should succeed");
    assert!(result_bytes.is_ok(), "parse_bytes should succeed");
    
    let xtab_file = result_file.unwrap();
    let xtab_str = result_str.unwrap();
    let xtab_bytes = result_bytes.unwrap();
    
    // Results should be equivalent
    assert_eq!(xtab_file.version, xtab_str.version);
    assert_eq!(xtab_file.version, xtab_bytes.version);
    
    assert_eq!(xtab_file.date, xtab_str.date);
    assert_eq!(xtab_file.date, xtab_bytes.date);
    
    assert_eq!(xtab_file.time, xtab_str.time);
    assert_eq!(xtab_file.time, xtab_bytes.time);
    
    assert_eq!(xtab_file.user, xtab_str.user);
    assert_eq!(xtab_file.user, xtab_bytes.user);
    
    assert_eq!(xtab_file.tables.len(), xtab_str.tables.len());
    assert_eq!(xtab_file.tables.len(), xtab_bytes.tables.len());
    
    // Compare first table details
    if !xtab_file.tables.is_empty() {
        let table_file = &xtab_file.tables[0];
        let table_str = &xtab_str.tables[0];
        let table_bytes = &xtab_bytes.tables[0];
        
        assert_eq!(table_file.title, table_str.title);
        assert_eq!(table_file.title, table_bytes.title);
        
        assert_eq!(table_file.name, table_str.name);
        assert_eq!(table_file.name, table_bytes.name);
        
        assert_eq!(table_file.statistics.len(), table_str.statistics.len());
        assert_eq!(table_file.statistics.len(), table_bytes.statistics.len());
        
        assert_eq!(table_file.data.rows.len(), table_str.data.rows.len());
        assert_eq!(table_file.data.rows.len(), table_bytes.data.rows.len());
    }
}

#[test]
fn test_parse_str_with_utf8() {
    // Test that parsing handles UTF-8 correctly
    let xml_with_utf8 = r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<xtab version="1.1" xmlns:xt="http://www.XtabML.org/2005/xtab" xmlns="http://www.XtabML.org/2005/xtab">
  <date>16/11/2025</date>
  <user>Miles</user>
  <controltype name="project" status="primary">
    <t>Project</t>
  </controltype>
  <control type="project">
    <t>Test with special chars: café naïve résumé</t>
  </control>
  <table name="test-utf8">
    <t>UTF-8 Test</t>
    <edge axis="r">
      <group>
        <element>
          <t>Row 1: 🚀</t>
        </element>
      </group>
    </edge>
    <edge axis="c">
      <group>
        <element>
          <t>Col 1: 🌟</t>
        </element>
      </group>
    </edge>
    <statistic type="Values" />
    <data>
      <r>
        <c>
          <v>100.0%</v>
        </c>
      </r>
    </data>
  </table>
</xtab>"#;
    
    let result = XtabMLParser::parse_str(xml_with_utf8);
    assert!(result.is_ok(), "Should parse UTF-8 content correctly");
    
    let xtab = result.unwrap();
    assert_eq!(xtab.version, "1.1");
    assert!(!xtab.tables.is_empty());
    let table = &xtab.tables[0];
    
    // Parser doesn't parse controls, so we check table title instead
    assert_eq!(table.title, "UTF-8 Test");
    
    let row_labels = table.row_labels();
    assert!(row_labels.iter().any(|label| label.contains("🚀")));
    
    let column_labels = table.column_labels();
    assert!(column_labels.iter().any(|label| label.contains("🌟")));
}

#[test]
fn test_large_file_parsing() {
    // Test that the parser can handle the relatively large example file
    let file_content = fs::read_to_string(&example_file_path())
        .expect("Should be able to read example file");
    
    // Check that the file is reasonably large
    assert!(file_content.len() > 10000, "Example file should be substantial");
    
    let start_time = std::time::Instant::now();
    let result = XtabMLParser::parse_str(&file_content);
    let duration = start_time.elapsed();
    
    assert!(result.is_ok(), "Should parse large file successfully");
    assert!(duration.as_secs() < 5, "Parsing should complete within reasonable time");
    
    let xtab = result.unwrap();
    assert!(!xtab.tables.is_empty(), "Should parse tables from large file");
}
