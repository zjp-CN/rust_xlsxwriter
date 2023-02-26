// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    worksheet.write_string(6, 0, "Pear")?;

    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(45925120, 45927040);
    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));
    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_name("Apple");
    chart
        .add_series()
        .set_values(("Sheet1", 0, 2, 4, 2))
        .set_name("=Sheet1!$A$7");

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

// Test to demonstrate charts.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    worksheet.write_string(6, 0, "Pear")?;

    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(45925120, 45927040);
    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));
    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_name("Apple");
    chart
        .add_series()
        .set_values(("Sheet1", 0, 2, 4, 2))
        .set_name(("Sheet1", 6, 0));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar20_1() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar20")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_chart_bar20_2() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar20")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
