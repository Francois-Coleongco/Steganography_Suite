use cli_entry::{add_entry, read_entry_handler};

mod cli_entry;

#[tauri::command]
fn invoke_add_entry(master_password: String, data: String, file_path: String) -> Result<String, String> {
    add_entry(master_password, data, file_path)
}

#[tauri::command]
fn invoke_read_entry(master_password: String, file_path: String) -> Result<String, String> {
    read_entry_handler(master_password, &file_path)
}

#[tauri::command]
fn invoke_hex_diff(original_path: String, stego_path: String) -> Result<String, String> {
    let original = std::fs::read(&original_path)
        .map_err(|e| format!("Failed to read original: {}", e))?;
    let stego = std::fs::read(&stego_path)
        .map_err(|e| format!("Failed to read stego: {}", e))?;

    let orig_name = std::path::Path::new(&original_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let stego_name = std::path::Path::new(&stego_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let max_len = original.len().max(stego.len());
    let mut diff_count = 0usize;
    let mut diff_lines: Vec<String> = Vec::new();

    for i in 0..max_len {
        let o = original.get(i).copied();
        let s = stego.get(i).copied();
        if o != s {
            diff_count += 1;
            let o_hex = o.map(|b| format!("{:02X}", b)).unwrap_or_else(|| "  --".to_string());
            let s_hex = s.map(|b| format!("{:02X}", b)).unwrap_or_else(|| "  --".to_string());
            let o_ascii = o.map(|b| ascii_char(b)).unwrap_or(' ');
            let s_ascii = s.map(|b| ascii_char(b)).unwrap_or(' ');
            diff_lines.push(format!(
                "{:08X}   {}    {}      {} {}",
                i, o_hex, s_hex, o_ascii, s_ascii
            ));
        }
    }

    let pct = if original.len() > 0 {
        (diff_count as f64 / original.len() as f64) * 100.0
    } else {
        0.0
    };

    let mut out = String::new();
    out.push_str("FILE COMPARISON\n");
    out.push_str(&format!("  original: {} ({:} bytes)\n", orig_name, original.len()));
    out.push_str(&format!("  stego:    {} ({:} bytes)\n", stego_name, stego.len()));
    out.push_str(&format!("  changed:  {} bytes ({:.2}%)\n", diff_count, pct));

    if diff_lines.is_empty() {
        out.push_str("\n  no differences found\n");
    } else {
        out.push_str("\n");
        out.push_str("OFFSET     ORIG  STEGO   ASCII\n");
        out.push_str("─────────  ────  ────    ─────\n");
        let display_limit = 500;
        for line in diff_lines.iter().take(display_limit) {
            out.push_str(line);
            out.push('\n');
        }
        if diff_lines.len() > display_limit {
            out.push_str(&format!(
                "\n... {} more differences omitted\n",
                diff_lines.len() - display_limit
            ));
        }
    }

    Ok(out)
}

#[tauri::command]
fn invoke_hex_dump(file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut out = String::new();
    out.push_str(&format!("HEX DUMP: {} ({:} bytes)\n", file_name, data.len()));
    out.push_str("\n");
    out.push_str("OFFSET     00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F  ASCII\n");
    out.push_str("─────────  ────────────────────────  ────────────────────────  ────────────────\n");

    for (offset, chunk) in data.chunks(16).enumerate() {
        let addr = offset * 16;
        let mut hex_part = String::new();
        let mut ascii_part = String::new();

        for (i, byte) in chunk.iter().enumerate() {
            if i == 8 {
                hex_part.push(' ');
            }
            hex_part.push_str(&format!("{:02X} ", byte));
            ascii_part.push(ascii_char(*byte));
        }

        // Pad if last chunk is shorter than 16 bytes
        if chunk.len() < 16 {
            let remaining = 16 - chunk.len();
            for i in 0..remaining {
                if i == (8usize.saturating_sub(chunk.len().min(8))) {
                    hex_part.push(' ');
                }
                hex_part.push_str("   ");
            }
        }

        out.push_str(&format!("{:08X}   {:<48}  {}\n", addr, hex_part, ascii_part));
    }

    Ok(out)
}

fn ascii_char(b: u8) -> char {
    if (0x20..=0x7E).contains(&b) {
        b as char
    } else {
        '.'
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            invoke_add_entry,
            invoke_read_entry,
            invoke_hex_diff,
            invoke_hex_dump
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
