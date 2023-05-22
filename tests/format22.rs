// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate use of the Automatic color.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format1 = Format::new()
        .set_font_color(Color::Automatic)
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::Automatic);

    worksheet.write_with_format(0, 0, "Foo", &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format22() {
    let test_runner = common::TestRunner::new()
        .set_name("format22")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
