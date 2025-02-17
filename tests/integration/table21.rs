// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableColumn, Workbook, XlsxError};

// Write a table with a user specified header.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for col_num in 2..=3u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    worksheet.write(0, 0, "Column")?;

    let mut table = Table::new();
    let columns = vec![TableColumn::new().set_header("Column")];

    table.set_columns(&columns);

    worksheet.add_table(2, 2, 12, 3, &table)?;

    workbook.save(filename)?;

    Ok(())
}

// Write a table that takes the header name from the worksheet cell data.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for col_num in 2..=3u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    worksheet.write(0, 0, "Column")?;

    // Write the header string, the table should read this and add it.
    worksheet.write(2, 2, "Column")?;

    let table = Table::new();

    worksheet.add_table(2, 2, 12, 3, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table21_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table21")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_table21_2() {
    let test_runner = common::TestRunner::new()
        .set_name("table21")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
