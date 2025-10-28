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
    println!("Title: {}, UUID: {:?}", table.title, table.name.as_ref());
        
        println!("Controls:");
        //for control in &table.controls {
        //    println!("  - {}: {}", control.r#type, control.text);
        //}i
        println!("{:?}", &table);
        for row in &table.data.rows {
            print!("|");
            for cell in &row.cells {
                print!("{:}", cell.value.clone().unwrap_or(" ".to_string()));
            }
            println!("|");
        }
    }    
    Ok(())
}

