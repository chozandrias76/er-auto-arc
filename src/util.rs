use std::slice;

use broadsword::{runtime, scanner};

/// Takes an instruction pattern and looks for its location
pub fn match_instruction_pattern(pattern: &str) -> Option<PatternResult> {
    // Find .text section details since that's where the code lives
    let text_section = runtime::get_module_section_range("eldenring.exe", ".text")
        .or_else(|_| runtime::get_module_section_range("start_protected_game.exe", ".text"))
        .unwrap();

    // Represent search area as a slice
    let scan_slice = unsafe {
        slice::from_raw_parts(
            text_section.start as *const u8,
            text_section.end - text_section.start,
        )
    };
    log::info!("Scanning for pattern: {}", pattern);
    let pattern = scanner::Pattern::from_bit_pattern(pattern).unwrap();

    scanner::simple::scan(scan_slice, &pattern)
        // TODO: this kinda of rebasing can be done in broadsword probably
        .map(|result| PatternResult {
            location: text_section.start + result.location,
            captures: result.captures.into_iter()
                .map(|capture| {
                    PatternCapture {
                        location: text_section.start + capture.location,
                        bytes: capture.bytes,
                    }
                })
                .collect()
        })
}

#[derive(Debug)]
pub struct PatternResult {
    pub location: usize,
    pub captures: Vec<PatternCapture>,
}

#[derive(Debug)]
pub struct PatternCapture {
    pub location: usize,
    pub bytes: Vec<u8>,
}
