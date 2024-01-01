// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates serializing instances of a Serde derived
//! data structure to a worksheet.

use rust_xlsxwriter::{
    CustomSerializeHeader, Format, SerializeHeadersOptions, Workbook, XlsxError,
};
use serde::Serialize;

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Add some formats to use with the serialization data.
    let bold = Format::new().set_bold();
    let currency = Format::new().set_num_format("$0.00");

    // Create a serializable struct.
    #[derive(Serialize)]
    struct Produce {
        fruit: &'static str,
        cost: f64,
    }

    // Create some data instances.
    let items = [
        Produce {
            fruit: "Peach",
            cost: 1.05,
        },
        Produce {
            fruit: "Plum",
            cost: 0.15,
        },
        Produce {
            fruit: "Pear",
            cost: 0.75,
        },
    ];

    // Set up the start location and headers of the data to be serialized using
    // custom headers.
    let custom_headers = [
        CustomSerializeHeader::new("fruit").rename("Fruit"),
        CustomSerializeHeader::new("cost")
            .rename("Price")
            .set_value_format(&currency),
    ];
    let header_options = SerializeHeadersOptions::new()
        .set_header_format(&bold)
        .set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &items[0], &header_options)?;

    // Serialize the data.
    worksheet.serialize(&items)?;

    // Save the file.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
