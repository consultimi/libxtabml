# libxtabml - Architecture Documentation

## Overview

`libxtabml` is a Rust library for parsing XtabML (Survey Table Interchange Format) files. XtabML is an XML-based format for describing survey cross-tabulation tables, commonly used in market research and survey analysis.

See [https://ascconference.org/xtabml-survey-table-interchange-format/] for more information.

This parser is a work-in-progress, parsing works but is not yet perfect.

## Project Structure

```
libxtabml/
├── Cargo.toml          # Project dependencies and configuration
├── README.md           # User-facing documentation
├── ARCHITECTURE.md     # This file
├── src/
│   ├── lib.rs          # Library root, exports public API
│   ├── types.rs        # Data structures representing XtabML
│   └── parser.rs       # XML parsing implementation
├── examples/
│   └── basic.rs        # Basic Example usage
│   └── texttables.rs   # Prints tables from the example in plaintext
└── resources/
    ├── example.xte      # Sample XtabML file
    └── XtabML-specification_1.1.pdf  # Specification reference
```

## Components

### 1. Types Module (`src/types.rs`)

Defines all data structures that represent an XtabML document:

- **XtabML**: Root structure containing document metadata and tables
- **Table**: Individual cross-tabulation table with edges, controls, and data
- **Edge**: Row or column dimension definition
- **Group**: Container for elements in an edge
- **Element**: Individual item (label) in a group
- **Control**: Metadata (weight, base, etc.)
- **DataCell**: Individual cell value or missing indicator
- DataRowSeries: A collection of cells defined by a single statistic (e.g. ColumnPercent) 
- DataRow: A collection of DataRowSeries

All types derive `Serialize` and `Deserialize` for JSON/other format support.

### 2. Parser Module (`src/parser.rs`)

Implements XML parsing using `quick-xml`:

- **XtabMLParser**: Main parser struct
- State machine approach tracking:
  - Current table, edge, group being parsed
  - Text buffers for accumulating element content
  - Path stack for context
- Handles the nested XML structure efficiently
- Extracts all tables, controls, and data

### 3. Library Root (`src/lib.rs`)

- Defines `XtabMLError` for error handling
- Exports public API
- Defines `Result<T>` type alias

## Data Flow

1. **Parse**: Read XML file and build XtabML structure
2. **Access**: Use provided methods on Table to extract data
3. **Process**: Work with parsed data structures

## Key Features

### Multiple Statistics Support

Tables can contain multiple statistics (e.g., Percent, n, ColumnPercent). The data rows are interleaved where row 0, 2, 4... might be percentages and rows 1, 3, 5... might be sample sizes.

Use `table.get_statistic_data(statistic_index)` to extract data for a specific statistic.

### Missing Values

Cells can be marked as missing using `<x />` elements. These are represented as `DataCell` with `is_missing = true`.

### Metadata Extraction

Tables include:
- Title and optional UUID name
- Controls (weight, base information)
- Row and column labels
- Statistic types

## Example Usage

```rust
use libxtabml::XtabMLParser;

// Parse file
let xtab = XtabMLParser::parse_file("survey.xte")?;

// Access tables
for table in &xtab.tables {
    println!("{}", table.title);
    
    // Get labels
    let rows = table.row_labels();
    let cols = table.column_labels();
    
    // Get data for first statistic
    if let Some(data) = table.get_statistic_data(0) {
        // data is Vec<Vec<Option<String>>>
        // Process your data...
    }
}
```

## Dependencies

- **quick-xml**: Fast XML parsing
- **serde**: Serialization support
- **thiserror**: Clean error handling

## Error Handling

Uses `thiserror` for type-safe error reporting:
- `XmlParse`: XML parsing errors
- `InvalidStructure`: Malformed XtabML structure
- `MissingElement`: Required element not found
- `Io`: File I/O errors

## Performance Considerations

- Uses `quick-xml` which is one of the fastest XML parsers in Rust
- Single-pass parsing
- Minimal allocations
- Lazy evaluation where possible

## Future Enhancements

Potential additions:
- Validation against XSD schema
- Export to other formats (CSV, Excel)
- Statistics calculation
- Table filtering/searching
- Incremental parsing for large files

