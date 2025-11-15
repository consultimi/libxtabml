#[test]
fn debug_parser_output() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
        .join("resources")
        .join("example.xte")
        .to_string_lossy()
        .to_string();
    
    let result = libxtabml::XtabMLParser::parse_file(&path);
    match result {
        Ok(xtab) => {
            println!("Successfully parsed XtabML file");
            println!("Version: {}", xtab.version);
            println!("Date: {:?}", xtab.date);
            println!("Time: {:?}", xtab.time);
            println!("User: {:?}", xtab.user);
            println!("Control types count: {}", xtab.control_types.len());
            println!("Statistic types count: {}", xtab.statistic_types.len());
            println!("Controls count: {}", xtab.controls.len());
            println!("Tables count: {}", xtab.tables.len());
            
            for (i, table) in xtab.tables.iter().enumerate() {
                println!("Table {} title: {}", i, table.title);
                println!("Table {} name: {:?}", i, table.name);
                println!("Table {} controls count: {}", i, table.controls.len());
                println!("Table {} statistics count: {}", i, table.statistics.len());
                
                if let Some(row_edge) = &table.row_edge {
                    println!("Table {} row edge groups count: {}", i, row_edge.groups.len());
                    if !row_edge.groups.is_empty() {
                        println!("Table {} row elements count: {}", i, row_edge.groups[0].elements.len());
                        if !row_edge.groups[0].elements.is_empty() {
                            println!("Table {} first row element: {}", i, row_edge.groups[0].elements[0].text);
                        }
                    }
                }
                
                if let Some(col_edge) = &table.column_edge {
                    println!("Table {} column edge groups count: {}", i, col_edge.groups.len());
                    if !col_edge.groups.is_empty() {
                        println!("Table {} column elements count: {}", i, col_edge.groups[0].elements.len());
                        println!("Table {} column summaries count: {}", i, col_edge.groups[0].summaries.len());
                        if !col_edge.groups[0].elements.is_empty() {
                            println!("Table {} first column element: {}", i, col_edge.groups[0].elements[0].text);
                        }
                        if !col_edge.groups[0].summaries.is_empty() {
                            println!("Table {} first column summary: {}", i, col_edge.groups[0].summaries[0].text);
                        }
                    }
                }
                
                println!("Table {} data rows count: {}", i, table.data.rows.len());
                if !table.data.rows.is_empty() {
                    println!("Table {} first row series count: {}", i, table.data.rows[0].data_row_series.len());
                    if !table.data.rows[0].data_row_series.is_empty() {
                        println!("Table {} first series cells count: {}", i, table.data.rows[0].data_row_series[0].cells.len());
                        if !table.data.rows[0].data_row_series[0].cells.is_empty() {
                            println!("Table {} first cell value: {:?}", i, table.data.rows[0].data_row_series[0].cells[0].value);
                            println!("Table {} first cell is_missing: {}", i, table.data.rows[0].data_row_series[0].cells[0].is_missing);
                        }
                    }
                }
                
                // Check for missing values in this table
                let mut missing_count = 0;
                for row in &table.data.rows {
                    for series in &row.data_row_series {
                        for cell in &series.cells {
                            if cell.is_missing {
                                missing_count += 1;
                            }
                        }
                    }
                }
                println!("Table {} missing values count: {}", i, missing_count);
                println!("---");
            }
        }
        Err(e) => {
            println!("Error parsing file: {:?}", e);
        }
    }
}