// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Formula, Table, TableColumn, TableFunction, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width(2, 10.288)?;
    worksheet.set_column_width(3, 10.288)?;
    worksheet.set_column_width(4, 10.288)?;
    worksheet.set_column_width(5, 10.288)?;

    let mut table = Table::new();
    let columns = vec![
        TableColumn::new().set_total_label("Total"),
        TableColumn::new().set_total_function(TableFunction::Custom(Formula::new("D5+D9"))),
        TableColumn::new().set_total_function(TableFunction::Custom("SUM([Column3])".into())),
        TableColumn::new().set_total_function(TableFunction::Count),
    ];

    table.set_columns(&columns);
    table.set_total_row(true);

    worksheet.add_table(2, 2, 13, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table32() {
    let test_runner = common::TestRunner::new()
        .set_name("table32")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
