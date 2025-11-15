use libxtabml::{XtabMLParser};

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
        .join("resources")
        .join("example.xte")
        .to_string_lossy()
        .to_string();
    
    let result = XtabMLParser::parse_file(&path);
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
            
            if !xtab.tables.is_empty() {
                let first_table = &xtab.tables[0];
                println!("First table title: {}", first_table.title);
                println!("First table name: {:?}", first_table.name);
                println!("First table controls count: {}", first_table.controls.len());
                println!("First table statistics count: {}", first_table.statistics.len());
                
                if let Some(row_edge) = &first_table.row_edge {
                    println!("Row edge groups count: {}", row_edge.groups.len());
                    if !row_edge.groups.is_empty() {
                        println!("Row elements count: {}", row_edge.groups[0].elements.len());
                        if !row_edge.groups[0].elements.is_empty() {
                            println!("First row element: {}", row_edge.groups[0].elements[0].text);
                        }
                    }
                }
                
                if let Some(col_edge) = &first_table.column_edge {
                    println!("Column edge groups count: {}", col_edge.groups.len());
                    if !col_edge.groups.is_empty() {
                        println!("Column elements count: {}", col_edge.groups[0].elements.len());
                        if !col_edge.groups[0].elements.is_empty() {
                            println!("First column element: {}", col_edge.groups[0].elements[0].text);
                        }
                    }
                }
                
                println!("Data rows count: {}", first_table.data.rows.len());
                if !first_table.data.rows.is_empty() {
                    println!("First row series count: {}", first_table.data.rows[0].data_row_series.len());
                    if !first_table.data.rows[0].data_row_series.is_empty() {
                        println!("First series cells count: {}", first_table.data.rows[0].data_row_series[0].cells.len());
                        if !first_table.data.rows[0].data_row_series[0].cells.is_empty() {
                            println!("First cell value: {:?}", first_table.data.rows[0].data_row_series[0].cells[0].value);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error parsing file: {:?}", e);
        }
    }
}