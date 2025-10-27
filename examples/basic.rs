use libxtabml::{XtabMLParser, Result};

fn main() -> Result<()> {
    // Parse the example file
    let xtab = XtabMLParser::parse_file("resources/example.xte")?;
    
    println!("=== XtabML Document ===\n");
    println!("Version: {}", xtab.version);
    println!("Date: {:?}", xtab.date);
    println!("Time: {:?}", xtab.time);
    println!("User: {:?}", xtab.user);
    println!("\nNumber of tables: {}", xtab.tables.len());
    
    // Analyze each table
    for (idx, table) in xtab.tables.iter().enumerate() {
        println!("\n--- Table {} ---", idx + 1);
        println!("Title: {}", table.title);
        
        if let Some(name) = &table.name {
            println!("Name (UUID): {}", name);
        }
        
        println!("Controls:");
        for control in &table.controls {
            println!("  - {}: {}", control.r#type, control.text);
        }
        
        println!("\nStatistics: {:?}", table.statistic_types());
        
        let (rows, cols) = table.shape();
        println!("\nShape: {} rows × {} columns", rows, cols);
        
        if let Some(row_edge) = &table.row_edge {
            if let Some(group) = row_edge.groups.first() {
                println!("\nRow labels (first {}):", group.elements.len().min(5));
                for (i, element) in group.elements.iter().take(5).enumerate() {
                    println!("  {}. {}", i + 1, element.text);
                }
            }
        }
        
        if let Some(col_edge) = &table.column_edge {
            if let Some(group) = col_edge.groups.first() {
                println!("\nColumn labels (first {}):", group.elements.len().min(5));
                for (i, element) in group.elements.iter().take(5).enumerate() {
                    println!("  {}. {}", i + 1, element.text);
                }
            }
        }
        
        // Sample some data
        println!("\nSample data from first statistic:");
        if let Some(data) = table.get_statistic_data(0) {
            let sample_rows = data.len().min(3);
            let sample_cols = if data.is_empty() { 0 } else { data[0].len().min(4) };
            
            for i in 0..sample_rows {
                print!("  ");
                for j in 0..sample_cols {
                    if let Some(val) = &data[i][j] {
                        print!("{}", val.chars().take(12).collect::<String>());
                    } else {
                        print!("<missing>");
                    }
                    if j < sample_cols - 1 {
                        print!(" | ");
                    }
                }
                println!();
            }
        }
    }
    
    Ok(())
}

