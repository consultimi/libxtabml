use crate::{types::*, Result, XtabMLError};
use quick_xml::events::Event;
use quick_xml::Reader;

/// Parser for XtabML documents
pub struct XtabMLParser;

impl XtabMLParser {
    /// Parse an XtabML file from a path
    pub fn parse_file(path: &str) -> Result<XtabML> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// Parse an XtabML document from a string
    pub fn parse_str(content: &str) -> Result<XtabML> {
        let bytes = content.as_bytes();
        Self::parse_bytes(bytes)
    }

    /// Parse an XtabML document from bytes
    pub fn parse_bytes(bytes: &[u8]) -> Result<XtabML> {
        let mut reader = Reader::from_reader(bytes);
        reader.trim_text(true);
        reader.check_end_names(true);
        reader.check_comments(true);

        let mut buf = Vec::new();
        let mut xtabml = XtabML {
            version: String::new(),
            date: None,
            time: None,
            origin: None,
            user: None,
            languages: Vec::new(),
            control_types: Vec::new(),
            statistic_types: Vec::new(),
            controls: Vec::new(),
            tables: Vec::new(),
        };

        let mut path_stack: Vec<String> = Vec::new();
        let mut text_buffer = String::new();

        // Table parsing state
        let mut current_table: Option<Table> = None;
        let mut current_edge: Option<Edge> = None;
        let mut current_group: Option<Group> = None;
        let mut current_data_row: Option<DataRowSeries> = None;
        let mut current_data_cell: Option<DataCell> = None;
        let mut row_buf: Vec<DataRowSeries> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref()).to_string();
                    path_stack.push(name_str.clone());

                    match name.as_ref() {
                        b"xtab" => {
                            for attr in e.attributes() {
                                let attr = attr.unwrap();
                                if attr.key.as_ref() == b"version" {
                                    xtabml.version = String::from_utf8(attr.value.to_vec())
                                        .map_err(|_| {
                                            XtabMLError::InvalidStructure(
                                                "Invalid UTF-8 in version".to_string(),
                                            )
                                        })?;
                                }
                            }
                        }
                        b"table" => {
                            let mut name = None;
                            for attr in e.attributes() {
                                let attr = attr.unwrap();
                                if attr.key.as_ref() == b"name" {
                                    name = Some(String::from_utf8(attr.value.to_vec()).map_err(
                                        |_| {
                                            XtabMLError::InvalidStructure(
                                                "Invalid UTF-8 in name".to_string(),
                                            )
                                        },
                                    )?);
                                }
                            }
                            current_table = Some(Table {
                                name,
                                title: String::new(),
                                controls: Vec::new(),
                                row_edge: None,
                                column_edge: None,
                                statistics: Vec::new(),
                                data: TableData { rows: Vec::new() },
                            });
                        }
                        b"control" => {
                            let mut control_type = String::new();
                            for attr in e.attributes() {
                                let attr = attr.unwrap();
                                if attr.key.as_ref() == b"type" {
                                    control_type =
                                        String::from_utf8(attr.value.to_vec()).map_err(|_| {
                                            XtabMLError::InvalidStructure(
                                                "Invalid UTF-8 in control type".to_string(),
                                            )
                                        })?;
                                }
                            }
                            text_buffer.clear();

                            // Read until end of control
                            let mut depth = 1;
                            loop {
                                match reader.read_event_into(&mut buf) {
                                    Ok(Event::Start(_)) => depth += 1,
                                    Ok(Event::End(_)) => {
                                        depth -= 1;
                                        if depth == 0 {
                                            break;
                                        }
                                    }
                                    Ok(Event::Text(e)) => match e.unescape() {
                                        Ok(text) => text_buffer.push_str(&text),
                                        Err(e) => return Err(XtabMLError::XmlParse(e)),
                                    },
                                    Ok(Event::Eof) => {
                                        return Err(XtabMLError::InvalidStructure(
                                            "Unexpected EOF in control".to_string(),
                                        ))
                                    }
                                    Err(e) => return Err(XtabMLError::XmlParse(e)),
                                    _ => {}
                                }
                                buf.clear();
                            }

                            let control = Control {
                                r#type: control_type.clone(),
                                text: text_buffer.clone(),
                            };

                            if let Some(ref mut table) = current_table {
                                table.controls.push(control);
                            } else {
                                xtabml.controls.push(control);
                            }
                            text_buffer.clear();
                            buf.clear();
                            continue;
                        }
                        b"edge" => {
                            let mut axis = String::new();
                            for attr in e.attributes() {
                                let attr = attr.unwrap();
                                if attr.key.as_ref() == b"axis" {
                                    axis =
                                        String::from_utf8(attr.value.to_vec()).map_err(|_| {
                                            XtabMLError::InvalidStructure(
                                                "Invalid UTF-8 in axis".to_string(),
                                            )
                                        })?;
                                }
                            }
                            current_edge = Some(Edge {
                                axis,
                                groups: Vec::new(),
                            });
                        }
                        b"group" => {
                            current_group = Some(Group {
                                elements: Vec::new(),
                                summaries: Vec::new(),
                            });
                        }
                        b"element" => {
                            text_buffer.clear();
                        }
                        b"summary" => {
                            text_buffer.clear();
                        }
                        b"statistic" => {
                            if let Some(ref mut table) = current_table {
                                let mut stat_type = String::new();
                                for attr in e.attributes() {
                                    let attr = attr.unwrap();
                                    if attr.key.as_ref() == b"type" {
                                        stat_type = String::from_utf8(attr.value.to_vec())
                                            .map_err(|_| {
                                                XtabMLError::InvalidStructure(
                                                    "Invalid UTF-8 in statistic type".to_string(),
                                                )
                                            })?;
                                    }
                                }
                                table.statistics.push(Statistic { r#type: stat_type });
                            }
                        }
                        b"r" => {
                            current_data_row = Some(DataRowSeries {
                                statistic: None,
                                cells: Vec::new(),
                            });
                        }
                        b"c" => {
                            current_data_cell = Some(DataCell::default());
                        }
                        b"v" => {
                            text_buffer.clear();
                        }
                        b"x" => {
                            // Empty element indicating missing value
                            if let Some(ref mut cell) = current_data_cell {
                                cell.is_missing = true;
                                cell.value = None;
                            }
                        }
                        _ => {}
                    }
                }

                Ok(Event::End(e)) => {
                    let name = e.name();
                    path_stack.pop();

                    match name.as_ref() {
                        b"t" => {
                            // Text element - use the buffer
                            let text = text_buffer.clone();
                            text_buffer.clear();

                            // Determine where to put the text based on context
                            if let Some(ref mut table) = current_table {
                                if table.title.is_empty() && path_stack.iter().any(|p| p == "table")
                                {
                                    table.title = text;
                                }
                            }
                        }
                        b"element" => {
                            if !text_buffer.is_empty() {
                                if let Some(ref mut group) = current_group {
                                    group.elements.push(Element {
                                        text: text_buffer.clone(),
                                        index: None,
                                    });
                                }
                                text_buffer.clear();
                            }
                        }
                        b"summary" => {
                            if !text_buffer.is_empty() {
                                if let Some(ref mut group) = current_group {
                                    group.summaries.push(Summary {
                                        text: text_buffer.clone(),
                                    });
                                }
                                text_buffer.clear();
                            }
                        }
                        b"group" => {
                            if let Some(group) = current_group.take() {
                                if let Some(ref mut edge) = current_edge {
                                    edge.groups.push(group);
                                }
                            }
                        }
                        b"edge" => {
                            if let Some(edge) = current_edge.take() {
                                if let Some(ref mut table) = current_table {
                                    if edge.axis == "r" {
                                        table.row_edge = Some(edge);
                                    } else if edge.axis == "c" {
                                        table.column_edge = Some(edge);
                                    }
                                }
                            }
                        }
                        b"c" => {
                            if let Some(cell) = current_data_cell.take() {
                                if let Some(ref mut row) = current_data_row {
                                    row.cells.push(cell);
                                }
                            }
                        }
                        b"v" => {
                            // Value element
                            if let Some(ref mut cell) = current_data_cell {
                                if !text_buffer.is_empty() {
                                    cell.value = Some(text_buffer.clone());
                                    cell.is_missing = false;
                                }
                                text_buffer.clear();
                            }
                        }
                        b"r" => {
                            if let Some(row) = current_data_row.take() {
                                row_buf.push(row);
                            }
                        }
                        b"table" => {
                            if let Some(table) = current_table.take() {
                                // now have a rowbuf full of rows, ensure we have (num_rows %
                                // num_statistics)=0
                                assert!(
                                    row_buf.len().is_multiple_of(table.statistics.len()),
                                    "Incorrect number of rows found given the number of statistics",
                                );
                                // TODO divide the buffer into DataRowSeries and push to table

                                xtabml.tables.push(table);
                            }
                        }
                        _ => {}
                    }
                }

                Ok(Event::Text(e)) => {
                    match e.unescape() {
                        Ok(text) => {
                            text_buffer.push_str(&text);
                        }
                        Err(e) => {
                            // Handle unescape error by using raw text
                            text_buffer.push_str(&e.to_string());
                        }
                    }
                }

                Ok(Event::Eof) => break,
                Err(e) => return Err(XtabMLError::XmlParse(e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(xtabml)
    }
}

/// Parse an XtabML file from a path
#[allow(dead_code)]
pub fn parse_file(path: &str) -> Result<XtabML> {
    XtabMLParser::parse_file(path)
}

/// Parse an XtabML document from a string
#[allow(dead_code)]
pub fn parse_str(content: &str) -> Result<XtabML> {
    XtabMLParser::parse_str(content)
}
