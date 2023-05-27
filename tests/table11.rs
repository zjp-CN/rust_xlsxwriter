// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Table, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let items = ["Foo", "Bar", "Baz", "Bop"];
    let data = [
        [1234, 2000, 4321],
        [1256, 4000, 4320],
        [2234, 3000, 4332],
        [1324, 1000, 4333],
    ];

    worksheet.write_column(2, 2, items)?;
    worksheet.write_row_matrix(2, 3, data)?;

    for col_num in 2..=5u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    let table = Table::new();
    worksheet.add_table(1, 2, 5, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table11() {
    let test_runner = common::TestRunner::new()
        .set_name("table11")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
