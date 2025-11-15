use std::thread::current;

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
        let mut current_data_row: Option<DataRow> = None;
        let mut current_data_row_series_index: usize = 0;
        let mut current_data_cell: Option<DataCell> = None;
        let mut current_element: Option<Element> = None;
        let mut current_element_index: i32 = 0;
        let mut current_statistic_type: Option<StatisticType> = None;

        loop {
            let event = reader.read_event_into(&mut buf);
            match event {
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
                            current_element = None;
                            current_element_index = 0;
                        }
                        b"element" => {
                            //text_buffer.clear();
                            current_element = Some(Element {
                                text: "".to_string(),
                                index: None,
                            })
                        }
                        b"summary" => {
                            text_buffer.clear();
                        }
                        // b"statistic" => {
                        //     if let Some(ref mut table) = current_table {
                        //         let mut stat_type = String::new();
                        //         for attr in e.attributes() {
                        //             let attr = attr.unwrap();
                        //             if attr.key.as_ref() == b"type" {
                        //                 stat_type = String::from_utf8(attr.value.to_vec())
                        //                     .map_err(|_| {
                        //                         XtabMLError::InvalidStructure(
                        //                             "Invalid UTF-8 in statistic type".to_string(),
                        //                         )
                        //                     })?;
                        //             }
                        //         }
                        //         table.statistics.push(Statistic { r#type: stat_type });
                        //     }
                        // }
                        b"r" => {
                            if let Some(ref table) = current_table {
                                current_data_row = Some(DataRow {
                                    data_row_series: table
                                        .statistics
                                        .iter()
                                        .map(|_stat| DataRowSeries {
                                            statistic: Some(_stat.clone()),
                                            cells: Vec::new(),
                                        })
                                        .collect(),
                                });
                                current_data_row_series_index = 0;
                            }
                        }
                        b"c" => {
                            // start a statistic entry for this row
                            // current_data_row_series_index will be used to access the right series
                        }
                        b"v" => {
                            // strt a cell
                            current_data_cell = Some(DataCell::default());
                            // println!("{:?}", e);
                            // for attr in e.attributes() {
                            //     println!("{:?}", attr);
                            // }
                        }
                        b"x" => {
                            // Empty element indicating missing value
                            if let Some(ref mut cell) = current_data_cell {
                                cell.is_missing = true;
                                cell.value = None;
                            }
                        }
                        b"statistictype" => {
                            for attr in e.attributes() {
                                let attr = attr.unwrap();
                                if attr.key.as_ref() == b"name" {
                                    xtabml.version = String::from_utf8(attr.value.to_vec())
                                        .map_err(|_| {
                                            XtabMLError::InvalidStructure(
                                                "Invalid UTF-8 in version".to_string(),
                                            )
                                        })?;
                                }
                            }

                            current_statistic_type = Some(StatisticType {
                                name: "".to_string(),
                                text: "".to_string(),
                            });
                        }
                        _ => {
                            //println!("UNMATCHED EVENT IN START: {:?}", name);
                        }
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
                            //println!("INSIDE TEXT WITH VALUE: {}", text);
                            // Determine where to put the text based on context
                            if let Some(ref mut table) = current_table {
                                if table.title.is_empty() && path_stack.iter().any(|p| p == "table")
                                {
                                    table.title = text;
                                } else if current_element.is_some() {
                                    if let Some(ref mut element) = current_element {
                                        element.text = text;
                                        element.index = Some(current_element_index);
                                        current_element_index += 1;
                                    }
                                    //println!(
                                    //    "INSIDE TEXT WITH GROUP: {:?} AND element: {:?}",
                                    //    current_group, current_
                                    //    element
                                    //);
                                } else if current_statistic_type.is_some() {
                                    if let Some(ref mut stattype) = current_statistic_type {
                                        stattype.text = text;
                                    }
                                }
                            }
                        }
                        b"element" => {
                            //if !text_buffer.is_empty() {
                            if let Some(ref mut group) = current_group {
                                if let Some(ref mut element) = current_element {
                                    group.elements.push(element.clone());
                                    current_element = None;
                                }
                            }
                            //text_buffer.clear();
                            //}
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
                                    // println!("INSIDE EDGE WITH TEXT BUFFER : {}", text_buffer);

                                    if edge.axis == "r" {
                                        table.row_edge = Some(edge);
                                    } else if edge.axis == "c" {
                                        table.column_edge = Some(edge);
                                    }
                                }
                            }
                        }
                        b"c" => {
                            current_data_row_series_index += 1;
                        }
                        b"v" => {
                            // Value element
                            //println!("{:?}", text_buffer);
                            if let Some(ref mut cell) = current_data_cell.take() {
                                if !text_buffer.is_empty() {
                                    cell.value = Some(text_buffer.clone());
                                    cell.is_missing = false;
                                }

                                if let Some(ref mut row) = current_data_row {
                                    if current_data_row_series_index < row.data_row_series.len() {
                                        row.data_row_series[current_data_row_series_index]
                                            .cells
                                            .push(cell.clone());
                                    }
                                }
                            }
                            text_buffer.clear();
                        }
                        b"r" => {
                            if let Some(row) = current_data_row.take() {
                                if let Some(ref mut table) = current_table {
                                    table.data.rows.push(row);
                                }
                            }
                        }
                        b"table" => {
                            if let Some(table) = current_table.take() {
                                // now have a rowbuf full of rows, ensure we have (num_rows %
                                // num_statistics)=0
                                // TODO divide the buffer into DataRowSeries and push to table

                                xtabml.tables.push(table);
                            }
                        }
                        _ => {
                            println!("Got unexpected key: {:?}", name);
                        }
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
                Ok(Event::Empty(e)) => {
                    let name = e.name();

                    match name.as_ref() {
                        b"statistic" => {
                            if let Some(ref mut table) = current_table {
                                let mut stat_type = String::new();
                                //println!("INSIDE STATISTICS: text_buffer is {}", text_buffer);
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
                        b"x" => {
                            // Empty element indicating missing value
                            let missing_cell = DataCell {
                                is_missing: true,
                                value: None,
                            };
                            if let Some(ref mut row) = current_data_row {
                                if current_data_row_series_index < row.data_row_series.len() {
                                    row.data_row_series[current_data_row_series_index]
                                        .cells
                                        .push(missing_cell);
                                }
                            }
                        }
                        _ => {
                            //println!("Got other name: {:?}", name.as_ref());
                        }
                    }
                    //println!("Got empty with attributes: {:?}", e.attributes());
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(XtabMLError::XmlParse(e)),
                _ => {
                    //println!("GOT UNMATCHED EVENT: {:?}", event);
                }
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
