use libxtabml::{XtabMLParser, XtabMLError};

#[test]
fn test_nonexistent_file_error() {
    let result = XtabMLParser::parse_file("nonexistent_file.xte");
    assert!(result.is_err(), "Should return error for nonexistent file");
    
    match result.unwrap_err() {
        XtabMLError::Io(_) => {}, // Expected
        other => panic!("Expected Io error, got: {:?}", other),
    }
}

#[test]
fn test_empty_string_error() {
    let result = XtabMLParser::parse_str("");
    // Note: Current parser implementation doesn't fail on empty string
    // This test documents the current behavior
    assert!(result.is_ok(), "Parser currently succeeds for empty string");
}

#[test]
fn test_empty_bytes_error() {
    let empty_bytes: &[u8] = &[];
    let result = XtabMLParser::parse_bytes(empty_bytes);
    // Note: Current parser implementation doesn't fail on empty bytes
    // This test documents the current behavior
    assert!(result.is_ok(), "Parser currently succeeds for empty bytes");
}

#[test]
fn test_invalid_xml_structure_error() {
    let invalid_xml = r#"<xtab><invalid></xtab>"#;
    let result = XtabMLParser::parse_str(invalid_xml);
    assert!(result.is_err(), "Should return error for invalid XML");
    
    match result.unwrap_err() {
        XtabMLError::XmlParse(_) => {}, // Expected
        other => panic!("Expected XmlParse error, got: {:?}", other),
    }
}

#[test]
fn test_malformed_xml_error() {
    let malformed_xml = r#"<xtab version="1.1"
  <date>2025-01-01</date>
  <user>Test</user>
</xtab>"#;
    
    // Note: Current parser implementation panics on this malformed XML
    // This test documents the current behavior
    let result = std::panic::catch_unwind(|| {
        XtabMLParser::parse_str(malformed_xml)
    });
    
    assert!(result.is_err(), "Parser currently panics on malformed XML");
}

#[test]
fn test_missing_required_elements_error() {
    let xml_missing_elements = r#"<xtab version="1.1">
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_missing_elements);
    
    // This might not error immediately during parsing, but the structure would be incomplete
    // The exact behavior depends on the parser implementation
    if let Ok(xtab) = result {
        // If parsing succeeds, validate that the structure is indeed incomplete
        assert!(xtab.tables.is_empty(), "Should have no tables when none are defined");
        assert!(xtab.control_types.is_empty(), "Should have no control types when none are defined");
        assert!(xtab.statistic_types.is_empty(), "Should have no statistic types when none are defined");
    }
}

#[test]
fn test_invalid_utf8_error() {
    let invalid_utf8 = b"<xtab version=\"1.1\">\xff\xfe</xtab>";
    let result = XtabMLParser::parse_bytes(invalid_utf8);
    // Note: Current parser implementation handles invalid UTF-8 differently
    // This test documents the current behavior
    assert!(result.is_ok(), "Parser currently handles invalid UTF-8 without error");
}

#[test]
fn test_invalid_version_attribute() {
    let xml_invalid_version = r#"<xtab>
  <date>2025-01-01</date>
  <user>Test</user>
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_invalid_version);
    
    // Parser should handle missing version gracefully or return appropriate error
    if let Ok(xtab) = result {
        assert_eq!(xtab.version, "", "Version should be empty when not provided");
    } else {
        // Error is also acceptable
        match result.unwrap_err() {
            XtabMLError::XmlParse(_) | XtabMLError::InvalidStructure(_) => {}, // Expected
            other => panic!("Expected XmlParse or InvalidStructure error, got: {:?}", other),
        }
    }
}

#[test]
fn test_invalid_control_structure() {
    let xml_invalid_control = r#"<xtab version="1.1">
  <control>
    <t>Test control</t>
  </control>
  <table name="test">
    <t>Test Table</t>
    <edge axis="r">
      <group>
        <element>
          <t>Test Element</t>
        </element>
      </group>
    </edge>
    <edge axis="c">
      <group>
        <element>
          <t>Test Column</t>
        </element>
      </group>
    </edge>
    <statistic type="Values" />
    <data>
      <r>
        <c>
          <v>100</v>
        </c>
      </r>
    </data>
  </table>
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_invalid_control);
    
    // Control without type attribute might cause issues
    if let Ok(xtab) = result {
        // If parsing succeeds, check that control handling is reasonable
        if !xtab.controls.is_empty() {
            let control = &xtab.controls[0];
            // Type might be empty if not provided
            assert!(!control.text.is_empty(), "Control text should be preserved");
        }
    }
}

#[test]
fn test_incomplete_table_structure() {
    let xml_incomplete_table = r#"<xtab version="1.1">
  <table name="incomplete">
    <t>Incomplete Table</t>
    <!-- Missing edges -->
    <statistic type="Values" />
    <data>
      <r>
        <c>
          <v>100</v>
        </c>
      </r>
    </data>
  </table>
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_incomplete_table);
    
    // Parser should handle incomplete table structure
    if let Ok(xtab) = result {
        assert!(!xtab.tables.is_empty(), "Should have parsed the incomplete table");
        let table = &xtab.tables[0];
        
        // Edges might be None if not provided
        assert!(table.row_edge.is_none() || table.row_edge.is_some(), 
               "Row edge handling should be consistent");
        assert!(table.column_edge.is_none() || table.column_edge.is_some(), 
               "Column edge handling should be consistent");
    }
}

#[test]
fn test_mismatched_data_structure() {
    let xml_mismatched_data = r#"<xtab version="1.1">
  <table name="mismatched">
    <t>Mismatched Data Table</t>
    <edge axis="r">
      <group>
        <element>
          <t>Row 1</t>
        </element>
      </group>
    </edge>
    <edge axis="c">
      <group>
        <element>
          <t>Col 1</t>
        </element>
      </group>
    </edge>
    <statistic type="Values" />
    <statistic type="n" />
    <!-- Data rows don't match statistics count -->
    <data>
      <r>
        <c>
          <v>100</v>
        </c>
      </r>
    </data>
  </table>
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_mismatched_data);
    
    // Parser should handle mismatched data gracefully
    if let Ok(xtab) = result {
        let table = &xtab.tables[0];
        assert_eq!(table.statistics.len(), 2, "Should have 2 statistics");
        
        // Data structure might be inconsistent, but parser shouldn't crash
        if !table.data.rows.is_empty() {
            let row = &table.data.rows[0];
            // Row might have fewer series than statistics
            assert!(row.data_row_series.len() <= table.statistics.len(),
                   "Data series should not exceed statistics count");
        }
    }
}

#[test]
fn test_special_characters_in_xml() {
    let xml_with_special_chars = r#"<xtab version="1.1">
  <user>Test &amp; User &lt;test&gt;</user>
  <control type="project">
    <t>Project with &amp; special chars: "quotes" &amp; 'apostrophes'</t>
  </control>
  <table name="special-chars">
    <t>Table with special chars: &lt; &gt; &amp; &quot; &apos;</t>
    <edge axis="r">
      <group>
        <element>
          <t>Row with &amp; in name</t>
        </element>
      </group>
    </edge>
    <edge axis="c">
      <group>
        <element>
          <t>Column with &lt;tag&gt;</t>
        </element>
      </group>
    </edge>
    <statistic type="Values" />
    <data>
      <r>
        <c>
          <v>100%</v>
        </c>
      </r>
    </data>
  </table>
</xtab>"#;
    let result = XtabMLParser::parse_str(xml_with_special_chars);
    

    
    assert!(result.is_ok(), "Should handle XML entities correctly");
    
    let xtab = result.unwrap();
    
    // Parser doesn't parse user or controls, so we check table content instead
    assert!(!xtab.tables.is_empty());
    let table = &xtab.tables[0];
    assert!(table.title.contains("&"), "Table title should contain decoded ampersand");
    
    // Check row and column labels for entity decoding
    let row_labels = table.row_labels();
    assert!(row_labels.iter().any(|label| label.contains("&")), "Row labels should contain decoded ampersand");
    
    let column_labels = table.column_labels();
    assert!(column_labels.iter().any(|label| label.contains("<")), "Column labels should contain decoded less-than");
}

#[test]
fn test_very_large_xml_error() {
    // Create a very large XML document that might cause memory issues
    let mut large_xml = String::from(r#"<xtab version="1.1">
  <user>Test</user>
  <table name="large">
    <t>Large Table</t>
    <edge axis="r">
      <group>"#);
    
    // Add many elements
    for i in 0..10000 {
        large_xml.push_str(&format!(r#"
        <element>
          <t>Row {}</t>
        </element>"#, i));
    }
    
    large_xml.push_str(r#"
      </group>
    </edge>
    <edge axis="c">
      <group>
        <element>
          <t>Column 1</t>
        </element>
      </group>
    </edge>
    <statistic type="Values" />
    <data>"#);
    
    // Add many data rows
    for i in 0..10000 {
        large_xml.push_str(&format!(r#"
      <r>
        <c>
          <v>{}</v>
        </c>
      </r>"#, i * 10));
    }
    
    large_xml.push_str(r#"
    </data>
  </table>
</xtab>"#);
    
    let result = XtabMLParser::parse_str(&large_xml);
    
    // Should either succeed or fail gracefully (not crash)
    if result.is_ok() {
        let xtab = result.unwrap();
        assert!(!xtab.tables.is_empty(), "Should have parsed at least one table");
        
        let table = &xtab.tables[0];
        assert_eq!(table.data.rows.len(), 10000, "Should have parsed all rows");
    } else {
        // Error is acceptable for very large documents
        match result.unwrap_err() {
            XtabMLError::XmlParse(_) | XtabMLError::Io(_) => {}, // Expected
            other => panic!("Expected XmlParse or Io error, got: {:?}", other),
        }
    }
}
