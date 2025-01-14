use std::{
    ffi::CStr,
    fs::{self, DirEntry, Metadata},
    os::unix::fs::{MetadataExt, PermissionsExt},
    time::{SystemTime, UNIX_EPOCH},
};

const PINK: &str = "\x1b[35m";
const ORANGE: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

pub fn ls(args: Vec<String>) {
    let mut result = String::new();
    let mut total_blocks = 0;

    let mut show_all = false;
    let mut long_format = false;
    let mut show_file_type = false;

    // Sprinkle the arguments
    for arg in args {
        match arg.as_str() {
            "-a" => show_all = true,
            "-l" => long_format = true,
            "-F" => show_file_type = true,
            _ => {
                eprintln!("ls: invalid option -- '{}'", arg);
                return;
            }
        }
    }

    // List the current directory files
    let current_dir = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ls: cannot access current directory: {}", e);
            return;
        }
    };

    let mut entries: Vec<DirEntry> = Vec::new();

    if show_all {
        if long_format {
            //For the long format, add the appropriate metadata
            if let Ok(metadata) = fs::metadata(".") {
                result.push_str(&print_metadata(&metadata));
                if show_file_type {
                    result.push_str("\x1b[35m./\x1b[0m\n");
                } else {
                    result.push_str("\x1b[35m.\x1b[0m\n");
                }
                total_blocks += metadata.blocks() / 2;
            }
            if let Ok(metadata) = fs::metadata("..") {
                result.push_str(&print_metadata(&metadata));
                if show_file_type {
                    result.push_str("\x1b[35m../\x1b[0m\n");
                } else {
                    result.push_str("\x1b[35m..\x1b[0m\n");
                }
                total_blocks += metadata.blocks() / 2;
            }
        } else {
            result.push_str("\x1b[35m.  ..\x1b[0m  ");
        }
    }

    for entry in current_dir {
        if let Ok(entry) = entry {
            entries.push(entry);
        }
    }

    // Sort the files in alphabetical order
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let mut color = String::new();

        if let Ok(metadata) = entry.metadata() {
            color = right_color(&metadata);
        }

        if !show_all && file_name.starts_with('.') {
            continue;
        }

        if long_format {
            if let Ok(metadata) = entry.metadata() {
                total_blocks += metadata.blocks() / 2;
                result.push_str(&print_metadata(&metadata));
            }
        }

        if show_file_type {
            if let Ok(metadata) = entry.metadata() {
                result.push_str(&print_file_type(&metadata, &file_name));
            } else {
                result.push_str(&format!("{}{}{}", color, file_name, RESET));
            }
        } else {
            result.push_str(&format!("{}{}{}", color, file_name, RESET));
        }

        if !long_format {
            result.push_str("  ");
        } else {
            result.push_str("\n");
        }
    }
    if !long_format {
        result.push_str("\n");
    } else {
        println!("total {}", total_blocks);
    }

    print!("{}", result);
}

fn print_metadata(metadata: &Metadata) -> String {
    let permissions = format_permissions(metadata);
    let link_count = get_link_count(metadata);
    let owner_group = get_owner_and_group(metadata);
    let size = metadata.len();

    let modified = match metadata.modified() {
        Ok(time) => format_time(time),
        Err(_) => "unknown".to_string(),
    };

    format!(
        "{} {:1} {:<8} {:>5} {} ",
        permissions, link_count, owner_group, size, modified,
    )
}

fn format_time(time: SystemTime) -> String {
    // get the seconds from Unix_Epoch
    let duration = match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(_) => return "date invalide".to_string(),
    };

    // Convert into date components
    let total_secs = duration.as_secs();
    let total_mins = total_secs / 60;
    let mins = total_mins % 60;
    let total_hours = total_mins / 60;
    let hours = total_hours % 24;

    // Calculate the days since 1970-01-01
    let days_since_epoch = (total_hours / 24) as u32;

    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mois = [
        "Jan", "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sept", "Oct", "Nov", "Dec",
    ];

    // Calculate the month and the day
    let mut remaining_days = days_since_epoch;

    // Adjust for bissextile years
    for y in 1970..=2024 {
        if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
            if remaining_days >= 366 {
                remaining_days -= 366;
            }
        } else if remaining_days >= 365 {
            remaining_days -= 365;
        }
        if remaining_days < 365 {
            break;
        }
    }

    // Find the month and the day
    let mut month = 0;
    for (i, &days) in days_in_month.iter().enumerate() {
        if remaining_days < days {
            month = i;
            break;
        }
        remaining_days -= days;
    }
    let day = remaining_days + 1;

    format!("{} {:>2} {:02}:{:02}", mois[month], day, hours % 24, mins)
}

fn right_color(metadata: &Metadata) -> String {
    if metadata.is_dir() {
        format!("{}", PINK)
    } else if metadata.permissions().mode() & 0o111 != 0 {
        format!("{}", ORANGE)
    } else if metadata.file_type().is_symlink() {
        format!("{}", CYAN)
    } else {
        format!("{}", RESET)
    }
}

fn print_file_type(metadata: &Metadata, file_name: &str) -> String {
    if metadata.is_dir() {
        format!("{}{}/{}", PINK, file_name, RESET)
    } else if metadata.permissions().mode() & 0o111 != 0 {
        format!("{}{}*{}", ORANGE, file_name, RESET)
    } else if metadata.file_type().is_symlink() {
        format!("{}{}@{}", CYAN, file_name, RESET)
    } else {
        format!("{}", file_name)
    }
}

fn format_permissions(metadata: &Metadata) -> String {
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    // File type
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.is_file() {
        '-'
    } else {
        '?'
    };

    // User permissions (Owner)
    let user = format!(
        "{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' }, // Read
        if mode & 0o200 != 0 { 'w' } else { '-' }, // Write
        if mode & 0o100 != 0 { 'x' } else { '-' }  // Execute
    );

    // PermissionsGroupe (group)
    let group = format!(
        "{}{}{}",
        if mode & 0o040 != 0 { 'r' } else { '-' }, // Read
        if mode & 0o020 != 0 { 'w' } else { '-' }, // Write
        if mode & 0o010 != 0 { 'x' } else { '-' }  // Execute
    );

    // PermissionsAutres (others)
    let others = format!(
        "{}{}{}",
        if mode & 0o004 != 0 { 'r' } else { '-' }, // Read
        if mode & 0o002 != 0 { 'w' } else { '-' }, // Write
        if mode & 0o001 != 0 { 'x' } else { '-' }  // Execute
    );

    //Combine the type of file and permissions
    format!("{}{}{}{}", file_type, user, group, others)
}

fn get_link_count(metadata: &Metadata) -> u64 {
    metadata.nlink()
}

fn get_owner_and_group(metadata: &Metadata) -> String {
    let uid = metadata.uid();
    let gid = metadata.gid();

    // Recover the owner's name (User)
    let user_name = unsafe {
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            "unknown".to_string()
        } else {
            let user = CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned();
            user
        }
    };

    // Recover the name of the group
    let group_name = unsafe {
        let gr = libc::getgrgid(gid);
        if gr.is_null() {
            "unknown".to_string()
        } else {
            let group = CStr::from_ptr((*gr).gr_name).to_string_lossy().into_owned();
            group
        }
    };

    format!("{} {}", user_name, group_name)
}
