use serde::{Deserialize, Serialize};

/// Root element of an XtabML document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XtabML {
    pub version: String,
    pub date: Option<String>,
    pub time: Option<String>,
    pub origin: Option<String>,
    pub user: Option<String>,
    
    /// Languages used in the document
    pub languages: Vec<Language>,
    
    /// Control types defined in the document
    pub control_types: Vec<ControlType>,
    
    /// Statistic types defined in the document
    pub statistic_types: Vec<StatisticType>,
    
    /// Report-level controls
    pub controls: Vec<Control>,
    
    /// Tables in the document
    pub tables: Vec<Table>,
}

/// Language specification for alternative texts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub lang: String,
    pub base: Option<String>,
    pub description: String,
}

/// Control type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlType {
    pub name: String,
    pub status: Option<String>,
    pub text: String,
}

/// Statistic type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticType {
    pub name: String,
    pub text: String,
}

/// Control element (metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub r#type: String,
    pub text: String,
}

/// A table in the XtabML document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: Option<String>,
    pub title: String,
    
    /// Controls specific to this table (e.g., weight, base)
    pub controls: Vec<Control>,
    
    /// Row edge (axis="r")
    pub row_edge: Option<Edge>,
    
    /// Column edge (axis="c")
    pub column_edge: Option<Edge>,
    
    /// Statistics included in this table
    pub statistics: Vec<Statistic>,
    
    /// The data matrix
    pub data: TableData,
}

/// Edge definition (row or column)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub axis: String, // "r" for row, "c" for column
    pub groups: Vec<Group>,
}

/// A group within an edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub elements: Vec<Element>,
    pub summaries: Vec<Summary>,
}

/// An element (item) in a group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub text: String,
    pub index: Option<i32>,
}

/// A summary element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub text: String,
}

/// Statistic specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistic {
    pub r#type: String,
}

/// Table data matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub rows: Vec<DataRow>,
}

/// Represents multiple data series in a row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRowSeries {
    pub statistic: Option<Statistic>,
    pub cells: Vec<DataCell>
}

/// A row in the data matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRow {
    pub data_row_series: Vec<DataRowSeries>,
}

/// A cell in the data matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCell {
    pub value: Option<String>,
    pub is_missing: bool,
}

impl Default for DataCell {
    fn default() -> Self {
        Self {
            value: None,
            is_missing: false,
        }
    }
}

/// Convenience structure for accessing table data by statistic type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticData {
    pub statistic_type: String,
    pub values: Vec<Vec<Option<String>>>,
}

impl Table {
    /// Get all statistic types in this table
    pub fn statistic_types(&self) -> Vec<&str> {
        self.statistics.iter().map(|s| s.r#type.as_str()).collect()
    }
    
    /// Get the shape of the table (rows, columns)
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.rows.len();
        let cols = if rows > 0 {
            self.data.rows[0].cells.len()
        } else {
            0
        };
        (rows, cols)
    }
    
    /// Get data for a specific statistic type
    pub fn get_statistic_data(&self, statistic_index: usize) -> Option<Vec<Vec<Option<String>>>> {
        if statistic_index >= self.statistics.len() {
            return None;
        }
        
        let statistics_count = self.statistics.len();
        let _values_per_cell = (self.data.rows.len() / statistics_count).max(1);
        
        let mut result = Vec::new();
        
        // Extract values for this statistic
        for (row_idx, row) in self.data.rows.iter().enumerate() {
            if row_idx % statistics_count == statistic_index {
                let cell_values: Vec<Option<String>> = row.cells.iter()
                    .map(|cell| {
                        if cell.is_missing {
                            None
                        } else {
                            cell.value.clone()
                        }
                    })
                    .collect();
                result.push(cell_values);
            }
        }
        
        Some(result)
    }
    
    /// Get row labels from the row edge
    pub fn row_labels(&self) -> Vec<String> {
        self.row_edge.as_ref()
            .and_then(|e| e.groups.first())
            .map(|g| g.elements.iter().map(|e| e.text.clone()).collect())
            .unwrap_or_default()
    }
    
    /// Get column labels from the column edge
    pub fn column_labels(&self) -> Vec<String> {
        self.column_edge.as_ref()
            .and_then(|e| e.groups.first())
            .map(|g| g.elements.iter().map(|e| e.text.clone()).collect())
            .unwrap_or_default()
    }
}

