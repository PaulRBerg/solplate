use semver::Version;
use std::{env, process};

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    // Use the most recent version of Solidity by default.
    let mut version: String = "0.8.29".to_string();

    // Use the provided version if one is provided.
    if let Some(arg) = env::args().nth(1) {
        // If the version is not a semver, exit with an error.
        if Version::parse(&arg).is_err() {
            eprintln!("Invalid version: {}", arg);
            process::exit(1);
        }
        version = arg;
    }

    // Generate the boilerplate Solidity code.
    let boilerplate = format!(
        "{spdx}\n{pragma}{version};\n\n{contract}\n{function}\n}}\n",
        spdx = "// SPDX-License-Identifier: UNLICENSED",
        pragma = "pragma solidity >=",
        version = version,
        contract = "contract Contract {",
        function = "    function foo() external {}",
    );

    // Print the boilerplate to the console.
    println!("{}", boilerplate);

    // Copy the Solidity boilerplate to the clipboard.
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(boilerplate).unwrap(); // Copy the header to clipboard.
}
