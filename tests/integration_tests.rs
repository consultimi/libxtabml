use libxtabml::{XtabMLParser, Result, XtabML, Control};
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

/// Helper function to load and parse the example file
fn parse_example_file() -> Result<XtabML> {
    XtabMLParser::parse_file(&example_file_path())
}

#[test]
fn test_parse_example_file_basic() {
    let result = parse_example_file();
    assert!(result.is_ok(), "Failed to parse example file: {:?}", result.err());
    
    let xtab = result.unwrap();
    
    // Test basic document properties
    assert_eq!(xtab.version, "1.1");
    assert_eq!(xtab.date, Some("27/10/2025".to_string()));
    assert_eq!(xtab.time, Some("11:47 AM".to_string()));
    assert_eq!(xtab.user, Some("Miles".to_string()));
    
    // Should have multiple tables
    assert!(!xtab.tables.is_empty(), "Should have at least one table");
    assert!(xtab.tables.len() >= 6, "Should have at least 6 tables based on example file");
}

#[test]
fn test_parse_example_file_control_types() {
    let xtab = parse_example_file().unwrap();
    
    // Should have control types defined
    assert!(!xtab.control_types.is_empty(), "Should have control types defined");
    
    // Check for specific expected control types
    let control_type_names: Vec<String> = xtab.control_types.iter()
        .map(|ct| ct.name.clone())
        .collect();
    
    assert!(control_type_names.contains(&"project".to_string()));
    assert!(control_type_names.contains(&"filter".to_string()));
    assert!(control_type_names.contains(&"weight".to_string()));
    assert!(control_type_names.contains(&"base".to_string()));
}

#[test]
fn test_parse_example_file_statistic_types() {
    let xtab = parse_example_file().unwrap();
    
    // Should have many statistic types defined
    assert!(!xtab.statistic_types.is_empty(), "Should have statistic types defined");
    assert!(xtab.statistic_types.len() > 50, "Should have many statistic types");
    
    // Check for specific expected statistic types
    let stat_type_names: Vec<String> = xtab.statistic_types.iter()
        .map(|st| st.name.clone())
        .collect();
    
    assert!(stat_type_names.contains(&"Values".to_string()));
    assert!(stat_type_names.contains(&"Percent".to_string()));
    assert!(stat_type_names.contains(&"ColumnPercent".to_string()));
    assert!(stat_type_names.contains(&"RowPercent".to_string()));
    assert!(stat_type_names.contains(&"n".to_string()));
    assert!(stat_type_names.contains(&"BaseN".to_string()));
}

#[test]
fn test_parse_example_file_controls() {
    let xtab = parse_example_file().unwrap();
    
    // Should have project-level controls
    assert!(!xtab.controls.is_empty(), "Should have project-level controls");
    
    // Check for project control
    let project_control: Vec<&Control> = xtab.controls.iter()
        .filter(|c| c.r#type == "project")
        .collect();
    assert_eq!(project_control.len(), 1, "Should have exactly one project control");
    assert_eq!(project_control[0].text, "S195 Regular Frappe and Renewal Concepts");
}

#[test]
fn test_parse_example_file_first_table() {
    let xtab = parse_example_file().unwrap();
    
    // Get the first table
    let first_table = &xtab.tables[0];
    
    // Check table properties
    assert_eq!(first_table.title, "FILTERS by CONCEPT_SEEN (X)");
    assert!(first_table.name.is_some(), "First table should have a name/UUID");
    assert_eq!(first_table.name.as_ref().unwrap(), "dd98e8ec-5288-48a0-9fcd-796b40f09fec");
    
    // Should have controls
    assert!(!first_table.controls.is_empty(), "First table should have controls");
    
    // Check for base control
    let base_control: Vec<&Control> = first_table.controls.iter()
        .filter(|c| c.r#type == "base")
        .collect();
    assert_eq!(base_control.len(), 1, "First table should have exactly one base control");
    assert!(base_control[0].text.contains("Total sample; Unweighted; base n = 624"));
    
    // Should have statistics
    assert!(!first_table.statistics.is_empty(), "First table should have statistics");
    
    // Check for expected statistics
    let stat_types: Vec<String> = first_table.statistics.iter()
        .map(|s| s.r#type.clone())
        .collect();
    assert!(stat_types.contains(&"ColumnPercent".to_string()));
    assert!(stat_types.contains(&"n".to_string()));
}

#[test]
fn test_parse_example_file_table_edges() {
    let xtab = parse_example_file().unwrap();
    let first_table = &xtab.tables[0];
    
    // Should have both row and column edges
    assert!(first_table.row_edge.is_some(), "First table should have row edge");
    assert!(first_table.column_edge.is_some(), "First table should have column edge");
    
    let row_edge = first_table.row_edge.as_ref().unwrap();
    let column_edge = first_table.column_edge.as_ref().unwrap();
    
    assert_eq!(row_edge.axis, "r");
    assert_eq!(column_edge.axis, "c");
    
    // Should have groups
    assert!(!row_edge.groups.is_empty(), "Row edge should have groups");
    assert!(!column_edge.groups.is_empty(), "Column edge should have groups");
    
    // Check row elements
    let row_group = &row_edge.groups[0];
    assert!(!row_group.elements.is_empty(), "Row group should have elements");
    
    let row_labels: Vec<String> = row_group.elements.iter()
        .map(|e| e.text.clone())
        .collect();
    assert!(row_labels.contains(&"Total".to_string()));
    assert!(row_labels.contains(&"Frequent".to_string()));
    assert!(row_labels.contains(&"Infrequent".to_string()));
    assert!(row_labels.contains(&"NET".to_string()));
    
    // Check column elements
    let column_group = &column_edge.groups[0];
    assert!(!column_group.elements.is_empty(), "Column group should have elements");
    
    let column_labels: Vec<String> = column_group.elements.iter()
        .map(|e| e.text.clone())
        .collect();
    assert!(column_labels.contains(&"BR20401 BM".to_string()));
    assert!(column_labels.contains(&"R20401 R1".to_string()));
    assert!(column_labels.contains(&"R20402 R2".to_string()));
    assert!(column_labels.contains(&"NET".to_string()));
}

#[test]
fn test_parse_example_file_table_data() {
    let xtab = parse_example_file().unwrap();
    let first_table = &xtab.tables[0];
    
    // Should have data
    assert!(!first_table.data.rows.is_empty(), "First table should have data rows");
    
    let (rows, cols) = first_table.shape();
    assert!(rows > 0, "Should have at least one row");
    assert!(cols > 0, "Should have at least one column");
    
    // Check that we have the expected number of data rows based on the example
    assert!(rows >= 16, "Should have at least 16 data rows based on example file");
    
    // Check data structure - each row should have data series
    for row in &first_table.data.rows {
        assert!(!row.data_row_series.is_empty(), "Each row should have data series");
        
        // Each series should have cells
        for series in &row.data_row_series {
            assert!(!series.cells.is_empty(), "Each series should have cells");
        }
    }
}

#[test]
fn test_parse_example_file_data_values() {
    let xtab = parse_example_file().unwrap();
    let first_table = &xtab.tables[0];
    
    // Get first data row
    let first_row = &first_table.data.rows[0];
    
    // Should have data series matching the statistics
    assert_eq!(first_row.data_row_series.len(), first_table.statistics.len());
    
    // Check first series (should correspond to ColumnPercent statistic)
    let first_series = &first_row.data_row_series[0];
    assert!(!first_series.cells.is_empty(), "First series should have cells");
    
    // Check that we have both percentage and count values
    // First row should have ColumnPercent values like "100.000%"
    let first_cell = &first_series.cells[0];
    assert!(!first_cell.is_missing, "First cell should not be missing");
    assert!(first_cell.value.is_some(), "First cell should have a value");
    assert!(first_cell.value.as_ref().unwrap().contains("%"), "First cell should contain percentage");
    
    // Second series should have count values like "624.000"
    let second_series = &first_row.data_row_series[1];
    let second_cell = &second_series.cells[0];
    assert!(!second_cell.is_missing, "Second cell should not be missing");
    assert!(second_cell.value.is_some(), "Second cell should have a value");
}

#[test]
fn test_parse_example_file_missing_values() {
    let xtab = parse_example_file().unwrap();
    
    // Find a table that should have missing values (based on the example, tables with <x/> elements)
    let table_with_missing = xtab.tables.iter()
        .find(|t| t.title.contains("TOP 2 BOX") || t.title.contains("TOP BOX"))
        .expect("Should find a table with missing values");
    
    // Look for missing values in the data
    let mut found_missing = false;
    for row in &table_with_missing.data.rows {
        for series in &row.data_row_series {
            for cell in &series.cells {
                if cell.is_missing {
                    found_missing = true;
                    assert!(cell.value.is_none(), "Missing cells should have no value");
                    break;
                }
            }
            if found_missing { break; }
        }
        if found_missing { break; }
    }
    
    assert!(found_missing, "Should find at least one missing value in the data");
}

#[test]
fn test_parse_example_file_table_methods() {
    let xtab = parse_example_file().unwrap();
    let first_table = &xtab.tables[0];
    
    // Test statistic_types method
    let stat_types = first_table.statistic_types();
    assert!(!stat_types.is_empty(), "Should have statistic types");
    assert!(stat_types.contains(&"ColumnPercent"), "Should contain ColumnPercent");
    assert!(stat_types.contains(&"n"), "Should contain n");
    
    // Test shape method
    let (rows, cols) = first_table.shape();
    assert!(rows > 0, "Shape should have positive rows");
    assert!(cols > 0, "Shape should have positive columns");
    
    // Test row_labels method
    let row_labels = first_table.row_labels();
    assert!(!row_labels.is_empty(), "Should have row labels");
    assert!(row_labels.iter().any(|s| s == "Total"), "Should contain Total row label");
    
    // Test column_labels method
    let column_labels = first_table.column_labels();
    assert!(!column_labels.is_empty(), "Should have column labels");
    assert!(column_labels.iter().any(|s| s == "NET"), "Should contain NET column label");
}
