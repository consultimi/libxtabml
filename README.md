# libxtabml

A Rust library for parsing XtabML (Survey Table Interchange Format) files. XtabML is an XML format for describing the structure and content of survey cross-tabulation tables.

## Features

- Parse XtabML v1.0 and v1.1 files
- Extract table metadata and controls
- Parse table structure (edges, groups, elements)
- Extract statistical data
- Type-safe data structures
- Serialization support with Serde

## Usage

### Basic Example

```rust
use libxtabml::{XtabMLParser, Result};

fn main() -> Result<()> {
    // Parse an XtabML file
    let xtab = XtabMLParser::parse_file("example.xte")?;
    
    println!("XtabML version: {}", xtab.version);
    println!("Date: {:?}", xtab.date);
    println!("Number of tables: {}", xtab.tables.len());
    
    // Access table data
    for table in &xtab.tables {
        println!("Table: {}", table.title);
        println!("  Statistics: {:?}", table.statistic_types());
        println!("  Shape: {:?}", table.shape());
        
        // Get row and column labels
        let row_labels = table.row_labels();
        let col_labels = table.column_labels();
        println!("  Rows: {}", row_labels.len());
        println!("  Columns: {}", col_labels.len());
    }
    
    Ok(())
}
```

### Accessing Table Data

```rust
use libxtabml::XtabMLParser;

let xtab = XtabMLParser::parse_file("data.xte")?;

// Access the first table
if let Some(table) = xtab.tables.first() {
    // Get all statistic types
    let stat_types = table.statistic_types();
    
    // Get data for a specific statistic (e.g., index 0 for the first statistic)
    if let Some(data) = table.get_statistic_data(0) {
        for (row_idx, row) in data.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                println!("Row {} Col {}: {:?}", row_idx, col_idx, value);
            }
        }
    }
}
```

## Data Structures

### XtabML

The root structure containing:
- Document metadata (version, date, time, user)
- Control and statistic type definitions
- Tables

### Table

Represents a single cross-tabulation table with:
- Title and name (optional UUID)
- Controls (weight, base, etc.)
- Row and column edges
- Statistical data

### Data Access

Tables store data as a matrix of cells. Each cell can contain:
- A string value
- A missing value indicator (`<x />`)

Multiple statistics can be stored by interleaving rows. Use `get_statistic_data()` to extract data for a specific statistic type.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
libxtabml = "0.1"
```

## References

- [XtabML Specification](https://ascconference.org/wp-content/uploads/2020/02/XtabML-specification_1.1.pdf)

## License

See LICENSE file.

