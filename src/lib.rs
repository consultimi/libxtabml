mod parser;
mod types;

pub use parser::XtabMLParser;
pub use types::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum XtabMLError {
    #[error("XML parsing error: {0}")]
    XmlParse(#[from] quick_xml::Error),
    
    #[error("Invalid XtabML structure: {0}")]
    InvalidStructure(String),
    
    #[error("Missing required element: {0}")]
    MissingElement(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, XtabMLError>;

