// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate escaping control characters in strings.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for i in 0u8..127 {
        worksheet.write_string_only(i as u32, 0, &(i as char).to_string())?;
    }

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_shared_strings01() {
    let test_runner = common::TestRunner::new("shared_strings01").initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
