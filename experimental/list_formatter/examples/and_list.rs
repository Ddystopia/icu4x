// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_list::{markers::And, ListFormatter, ListStyle};
use icu_locid_macros::langid;
use writeable::Writeable;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let provider = icu_testdata::get_static_provider();

    let list_formatter =
        ListFormatter::<And>::try_new(langid!("es"), &provider, ListStyle::Wide).unwrap();

    println!(
        "{}",
        list_formatter
            .format(["España", "Francia", "Suiza", "Italia"].iter())
            .writeable_to_string()
    );

    0
}
