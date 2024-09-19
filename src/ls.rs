use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};
use users::{get_user_by_uid, get_group_by_gid};

pub fn handle_ls(args: &[&str]) {
    let show_all = args.contains(&"-a");  // Show hidden files (starting with .)
    let long_format = args.contains(&"-l");  // Show detailed information
    let classify = args.contains(&"-F");  // Append a character for file types

    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    let mut total_blocks = 0;
    let mut file_entries = vec![];

    // Add `.` and `..` when `-a` is specified
    if show_all {
        // Current directory (.)
        if let Ok(metadata) = fs::metadata(".") {
            if long_format {
                total_blocks += metadata.blocks() as u64 / 2; // Convert to 512-byte blocks
            }
            file_entries.push((
                ".".to_string(),
                fs::canonicalize(".").unwrap(),
                metadata,
            ));
        }

        // Parent directory (..)
        if let Ok(metadata) = fs::metadata("..") {
            if long_format {
                total_blocks += metadata.blocks() as u64 / 2; // Convert to 512-byte blocks
            }
            file_entries.push((
                "..".to_string(),
                fs::canonicalize("..").unwrap(),
                metadata,
            ));
        }
    }

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".to_string());

            // Skip hidden files if -a is not provided
            if !show_all && file_name.starts_with('.') {
                continue;
            }

            let metadata = match entry.metadata() {
                Ok(meta) => meta,
                Err(e) => {
                    eprintln!("Error getting metadata: {}", e);
                    continue;
                }
            };

            // Count the blocks for the total when using the -l option
            if long_format {
                let blocks = metadata.blocks();
                total_blocks += blocks as u64 / 2; // Convert to 512-byte blocks
            }

            file_entries.push((file_name, path, metadata));
        }
    }

    if long_format && !file_entries.is_empty() {
        println!("total {}", total_blocks);
    }

    // Sorting files alphabetically by their name
   file_entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    for (file_name, path, metadata) in &file_entries {
        if long_format {
            // File type
            let file_type = if metadata.is_dir() {
                'd'
            } else if metadata.is_symlink() {
                'l'
            } else {
                '-'
            };

            // Permissions
            let mode = metadata.permissions().mode();
            let mode_str = format!(
                "{}{}{}{}{}{}{}{}{}",
                if mode & 0o400 != 0 { 'r' } else { '-' },
                if mode & 0o200 != 0 { 'w' } else { '-' },
                if mode & 0o100 != 0 { 'x' } else { '-' },
                if mode & 0o040 != 0 { 'r' } else { '-' },
                if mode & 0o020 != 0 { 'w' } else { '-' },
                if mode & 0o010 != 0 { 'x' } else { '-' },
                if mode & 0o004 != 0 { 'r' } else { '-' },
                if mode & 0o002 != 0 { 'w' } else { '-' },
                if mode & 0o001 != 0 { 'x' } else { '-' },
            );

            // Get user and group names
            let user_name = get_user_by_uid(metadata.uid())
                .map(|u| u.name().to_string_lossy().to_string())
                .unwrap_or_else(|| metadata.uid().to_string());

            let group_name = get_group_by_gid(metadata.gid())
                .map(|g| g.name().to_string_lossy().to_string())
                .unwrap_or_else(|| metadata.gid().to_string());

            // File size
            let size = metadata.len();

            // Modification time
            let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
            let datetime: DateTime<Local> = DateTime::from(modified);
            let formatted_date = datetime.format("%b %d %H:%M").to_string();

            print!(
                "{}{} {:>3} {:>8} {:>8} {:>5} {} ",
                file_type,
                mode_str,
                metadata.nlink(),
                user_name,
                group_name,
                size,
                formatted_date,
            );
        }

        // Classify
        if classify {
            if path.is_dir() {
                print!("{}/", file_name);
            } else if path.is_symlink() {
                print!("{}@", file_name);
            } else if metadata.permissions().mode() & 0o111 != 0 {
                print!("{}*", file_name); // Executable file
            } else {
                print!("{}", file_name);
            }
        } else {
            print!("{}", file_name);
        }

        if long_format {
            println!(); // New line for long format
        } else {
            print!("  "); // Space between file names
        }
    }

    if !long_format && !file_entries.is_empty() {
        println!(); // New line after listing if there are files
    }
}
