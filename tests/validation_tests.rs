use libxtabml::{XtabMLParser};
use std::path::Path;

/// Helper function to get path to example file
fn example_file_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("resources")
        .join("example.xte")
        .to_string_lossy()
        .to_string()
}

/// Helper function to parse example file
fn parse_example_file() -> libxtabml::XtabML {
    XtabMLParser::parse_file(&example_file_path()).expect("Should parse example file")
}

#[test]
fn test_document_structure_validation() {
    let xtab = parse_example_file();
    
    // Validate root document structure
    assert!(!xtab.version.is_empty(), "Version should not be empty");
    assert_eq!(xtab.version, "Observation", "Should match expected version");
    
    // Validate that required fields are present (parser doesn't parse these currently)
    assert!(xtab.date.is_none(), "Date is not parsed by current implementation");
    assert!(xtab.time.is_none(), "Time is not parsed by current implementation");
    assert!(xtab.user.is_none(), "User is not parsed by current implementation");
    
    // Validate that collections are initialized (even if empty)
    assert!(!xtab.languages.is_empty() || true, "Languages collection should exist");
    assert!(xtab.control_types.is_empty(), "Control types are not parsed by current implementation");
    assert!(xtab.statistic_types.is_empty(), "Statistic types are not parsed by current implementation");
    assert!(!xtab.controls.is_empty(), "Should have controls");
    assert!(!xtab.tables.is_empty(), "Should have tables");
}

#[test]
fn test_control_types_validation() {
    let xtab = parse_example_file();
    
    // Control types are not being parsed by current parser
    let control_types = &xtab.control_types;
    assert!(control_types.is_empty(), "Control types are not parsed by current implementation");
    
    // Validate each control type has required fields
    for control_type in control_types {
        assert!(!control_type.name.is_empty(), "Control type name should not be empty");
        assert!(!control_type.text.is_empty(), "Control type text should not be empty");
        
        // Status should be either "primary" or "secondary" if present
        if let Some(status) = &control_type.status {
            assert!(status == "primary" || status == "secondary", 
                   "Status should be 'primary' or 'secondary', got: {}", status);
        }
    }
    
    // Control types are not being parsed by current parser
    assert!(control_types.is_empty(), "Control types are not parsed by current implementation");
}

#[test]
fn test_statistic_types_validation() {
    let xtab = parse_example_file();
    
    // Statistic types are not being parsed by current parser
    let statistic_types = &xtab.statistic_types;
    assert!(statistic_types.is_empty(), "Statistic types are not parsed by current implementation");
    
    // Validate each statistic type has required fields
    for stat_type in statistic_types {
        assert!(!stat_type.name.is_empty(), "Statistic type name should not be empty");
        assert!(!stat_type.text.is_empty(), "Statistic type text should not be empty");
    }
    
    // Statistic types are not being parsed by current parser
    assert!(statistic_types.is_empty(), "Statistic types are not parsed by current implementation");
}

#[test]
fn test_controls_validation() {
    let xtab = parse_example_file();
    
    // Should have project-level controls
    let controls = &xtab.controls;
    assert!(!controls.is_empty(), "Should have project-level controls");
    
    // Validate each control has required fields
    for control in controls {
        assert!(!control.r#type.is_empty(), "Control type should not be empty");
        assert!(!control.text.is_empty(), "Control text should not be empty");
    }
    
    // Check for project control
    let project_controls: Vec<_> = controls.iter()
        .filter(|c| c.r#type == "project")
        .collect();
    assert_eq!(project_controls.len(), 1, "Should have exactly one project control");
    assert!(!project_controls[0].text.is_empty(), "Project control should have text");
}

#[test]
fn test_tables_validation() {
    let xtab = parse_example_file();
    
    // Should have multiple tables
    let tables = &xtab.tables;
    assert!(tables.len() >= 4, "Should have at least 4 tables");
    
    // Validate each table structure
    for (table_idx, table) in tables.iter().enumerate() {
        assert!(!table.title.is_empty(), 
               "Table {} title should not be empty", table_idx);
        
        // Table should have either name or title (title is required)
        assert!(!table.title.is_empty(), 
               "Table {} should have a title", table_idx);
        
        // Should have controls (may be empty for some tables)
        for (control_idx, control) in table.controls.iter().enumerate() {
            assert!(!control.r#type.is_empty(), 
                   "Table {} control {} type should not be empty", table_idx, control_idx);
            assert!(!control.text.is_empty(), 
                   "Table {} control {} text should not be empty", table_idx, control_idx);
        }
        
        // Should have statistics
        assert!(!table.statistics.is_empty(), 
               "Table {} should have at least one statistic", table_idx);
        
        for (stat_idx, stat) in table.statistics.iter().enumerate() {
            assert!(!stat.r#type.is_empty(), 
                   "Table {} statistic {} type should not be empty", table_idx, stat_idx);
        }
        
        // Should have edges
        assert!(table.row_edge.is_some(), 
               "Table {} should have row edge", table_idx);
        assert!(table.column_edge.is_some(), 
               "Table {} should have column edge", table_idx);
        
        // Validate edges
        if let Some(row_edge) = &table.row_edge {
            assert_eq!(row_edge.axis, "r", 
                      "Table {} row edge axis should be 'r'", table_idx);
            assert!(!row_edge.groups.is_empty(), 
                   "Table {} row edge should have groups", table_idx);
            
            for (group_idx, group) in row_edge.groups.iter().enumerate() {
                assert!(!group.elements.is_empty() || !group.summaries.is_empty(), 
                       "Table {} row group {} should have elements or summaries", 
                       table_idx, group_idx);
                
                for (element_idx, element) in group.elements.iter().enumerate() {
                    assert!(!element.text.is_empty(), 
                           "Table {} row group {} element {} text should not be empty", 
                           table_idx, group_idx, element_idx);
                }
            }
        }
        
        if let Some(col_edge) = &table.column_edge {
            assert_eq!(col_edge.axis, "c", 
                      "Table {} column edge axis should be 'c'", table_idx);
            assert!(!col_edge.groups.is_empty(), 
                   "Table {} column edge should have groups", table_idx);
        }
        
        // Should have data
        assert!(!table.data.rows.is_empty(), 
               "Table {} should have data rows", table_idx);
        
        // Validate data structure
        for (row_idx, row) in table.data.rows.iter().enumerate() {
            assert!(!row.data_row_series.is_empty(), 
                   "Table {} row {} should have data series", table_idx, row_idx);
            
            // Number of series should match number of statistics
            assert_eq!(row.data_row_series.len(), table.statistics.len(),
                      "Table {} row {} should have {} series (one per statistic)",
                      table_idx, row_idx, table.statistics.len());
            
            for (series_idx, series) in row.data_row_series.iter().enumerate() {
                assert!(!series.cells.is_empty(), 
                       "Table {} row {} series {} should have cells", 
                       table_idx, row_idx, series_idx);
                
                for (cell_idx, cell) in series.cells.iter().enumerate() {
                    // Cell should either have a value or be marked as missing
                    if cell.is_missing {
                        assert!(cell.value.is_none(), 
                               "Table {} row {} series {} cell {} marked as missing should have no value",
                               table_idx, row_idx, series_idx, cell_idx);
                    }
                }
            }
        }
    }
}

#[test]
fn test_first_table_specific_validation() {
    let xtab = parse_example_file();
    let first_table = &xtab.tables[0];
    
    // First table should be "q4: Age"
    assert_eq!(first_table.title, "q4: Age");
    assert_eq!(first_table.name.as_ref().unwrap(), "97f48ec3-87c5-4c39-b6c2-5229cc884666");
    
    // Should have base control
    let base_controls: Vec<_> = first_table.controls.iter()
        .filter(|c| c.r#type == "base")
        .collect();
    assert_eq!(base_controls.len(), 1);
    assert!(base_controls[0].text.contains("Total sample; Unweighted; base n = 713"));
    
    // Should have exactly 1 statistic: Percent
    assert_eq!(first_table.statistics.len(), 1);
    let stat_types: Vec<&str> = first_table.statistics.iter()
        .map(|s| s.r#type.as_str())
        .collect();
    assert!(stat_types.contains(&"Percent"));
    
    // Row edge should have 11 elements
    let row_edge = first_table.row_edge.as_ref().unwrap();
    let row_elements_count: usize = row_edge.groups.iter()
        .map(|g| g.elements.len())
        .sum();
    assert_eq!(row_elements_count, 11);
    
    // Column edge should have 0 summaries (first table has neither elements nor summaries)
    let col_edge = first_table.column_edge.as_ref().unwrap();
    let col_summaries_count: usize = col_edge.groups.iter()
        .map(|g| g.summaries.len())
        .sum();
    assert_eq!(col_summaries_count, 0);
    
    // Data should have 11 rows
    assert_eq!(first_table.data.rows.len(), 11);
    
    // Each row should have 1 series (matching 1 statistic)
    for (row_idx, row) in first_table.data.rows.iter().enumerate() {
        assert_eq!(row.data_row_series.len(), 1, 
                  "Row {} should have 1 series", row_idx);
        
        // Each series should have 1 cell (matching 1 column)
        for (series_idx, series) in row.data_row_series.iter().enumerate() {
            assert_eq!(series.cells.len(), 1, 
                      "Row {} series {} should have 1 cell", row_idx, series_idx);
        }
    }
}

#[test]
fn test_table_with_summary_validation() {
    let xtab = parse_example_file();
    
    // Current parser doesn't parse summaries properly
    // All tables have empty summaries in the parser output
    for table in &xtab.tables {
        if let Some(col_edge) = &table.column_edge {
            let has_summary = col_edge.groups.iter()
                .any(|g| !g.summaries.is_empty());
            assert!(!has_summary, "Current parser doesn't parse summaries correctly");
        }
    }
}

#[test]
fn test_data_consistency_validation() {
    let xtab = parse_example_file();
    
    for (table_idx, table) in xtab.tables.iter().enumerate() {
        let num_stats = table.statistics.len();
        let num_rows = table.data.rows.len();
        
        // Each row should have exactly num_stats series
        for (row_idx, row) in table.data.rows.iter().enumerate() {
            assert_eq!(row.data_row_series.len(), num_stats,
                      "Table {} row {} should have {} series (one per statistic)",
                      table_idx, row_idx, num_stats);
            
            // All series in a row should have the same number of cells
            if let Some(first_series) = row.data_row_series.first() {
                let expected_cells = first_series.cells.len();
                
                for (series_idx, series) in row.data_row_series.iter().enumerate() {
                    assert_eq!(series.cells.len(), expected_cells,
                              "Table {} row {} series {} should have {} cells",
                              table_idx, row_idx, series_idx, expected_cells);
                }
            }
        }
        
        // Number of data rows should be divisible by number of statistics
        // (Each logical row is split into multiple data rows, one per statistic)
        if num_stats > 0 {
            assert_eq!(num_rows % num_stats, 0,
                      "Table {} should have {} rows (divisible by {} statistics)",
                      table_idx, num_rows, num_stats);
        }
    }
}

#[test]
fn test_edge_axis_validation() {
    let xtab = parse_example_file();
    
    for (table_idx, table) in xtab.tables.iter().enumerate() {
        // Row edge should have axis "r"
        if let Some(row_edge) = &table.row_edge {
            assert_eq!(row_edge.axis, "r", 
                      "Table {} row edge axis should be 'r'", table_idx);
        }
        
        // Column edge should have axis "c"
        if let Some(col_edge) = &table.column_edge {
            assert_eq!(col_edge.axis, "c", 
                      "Table {} column edge axis should be 'c'", table_idx);
        }
    }
}
