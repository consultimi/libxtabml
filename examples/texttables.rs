use libxtabml::{Result, XtabMLParser};

fn main() -> Result<()> {
    // Parse the example file
    let xtab = XtabMLParser::parse_file("resources/example.xte")?;

    //println!("{:?}", xtab);
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
        for control in &table.controls {
            println!("  - {}: {}", control.r#type, control.text);
        }

        let mut row_labels: Vec<String> = Vec::new();
        let mut out: Vec<Vec<String>> = Vec::new();
        let mut row_header = Vec::from(["Label".to_string(), "Statistic".to_string()]);

        if let Some(row_edge) = &table.row_edge {
            row_labels = row_edge
                .groups
                .iter()
                .flat_map(|group| group.elements.iter().map(|x| x.text.clone()))
                .collect();
        }
        if let Some(col_edge) = &table.column_edge {
            // On tables without a banner, the row labels will be in the column edge
            // TODO: handle nested banners more elegantly
            if !row_labels.is_empty() {
                let mut col_labels: Vec<String> = col_edge
                    .groups
                    .iter()
                    .flat_map(|group| group.elements.iter().map(|x| x.text.clone()))
                    .collect();

                // if col_labels is empty here, just create a row_header with the value "Value"
                if col_labels.is_empty() {
                    row_header.push("Value".to_string())
                } else {
                    row_header.append(&mut col_labels);
                }
            } else {
                row_labels = col_edge.groups[0]
                    .elements
                    .iter()
                    .map(|x| x.text.clone())
                    .collect();
                row_header.push("Value".to_string());
            }

            //if col_labels.is_empty() {
            //    println!("Got here5");
            //    row_header.push("Value".to_string());
            //}
        } else {
            // There should never not be a column edge, but if so, just create a single header
            row_header.push("Value".to_string());
        }
        out.push(row_header);
        //println!("{:?}", &table.statistics);
        //println!("{:}", col_labels.join("|"));
        let mut row_labels_iter = row_labels.iter();
        for row in &table.data.rows {
            let row_label = row_labels_iter.next().unwrap_or(&"".to_owned()).clone();
            for row_data_series in &row.data_row_series {
                let mut row_data = vec![row_label.clone()];

                if let Some(statistic) = &row_data_series.statistic {
                    row_data.push(statistic.r#type.clone());
                } else {
                    row_data.push("".to_string());
                }
                let mut cell_data: Vec<String> = row_data_series
                    .cells
                    .iter()
                    .map(|x| x.value.clone().unwrap_or("".to_string()))
                    .collect();
                row_data.append(&mut cell_data);
                out.push(row_data);
            }
        }
        //println!("{:?}", out);
        text_tables::render(&mut std::io::stdout(), out).unwrap();
    }
    Ok(())
}
