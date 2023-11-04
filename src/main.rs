use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io};

use chrono::DateTime;
use chrono::Local;

fn main() -> io::Result<()> {
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let permissions = metadata.permissions();
        let mode = render_mode(&metadata, permissions.mode());
        let (user, group) = owner_and_group_of_file(&metadata);
        let size = metadata.size();
        let last_modified = metadata.modified().unwrap();
        let date_time: DateTime<Local> = last_modified.into();
        let path_buf = entry.path();
        let name = path_buf.file_name().unwrap().to_str().unwrap();
        if !name.starts_with('.') {
            println!(
                "{} {} {} {} {} {}",
                mode,
                size,
                user,
                group,
                date_time.format("%d %b %H:%M"),
                name
            );
        }
    }

    Ok(())
}

#[cfg(unix)]
fn owner_and_group_of_file(metadata: &Metadata) -> (String, String) {
    let uid = metadata.uid();
    let gid = metadata.gid();

    use nix::unistd::{Group, User};
    let user = User::from_uid(nix::unistd::Uid::from_raw(uid))
        .unwrap()
        .unwrap();
    let group = Group::from_gid(nix::unistd::Gid::from_raw(gid))
        .unwrap()
        .unwrap();

    (user.name, group.name)
}

fn render_mode(metadata: &fs::Metadata, mode: u32) -> String {
    let mut chars = vec!['-'; 9];

    // Read, write, execute permissions for owner, group, and others
    if mode & 0o400 != 0 {
        chars[0] = 'r'
    }
    if mode & 0o200 != 0 {
        chars[1] = 'w'
    }
    if mode & 0o100 != 0 {
        chars[2] = 'x'
    }

    if mode & 0o040 != 0 {
        chars[3] = 'r'
    }
    if mode & 0o020 != 0 {
        chars[4] = 'w'
    }
    if mode & 0o010 != 0 {
        chars[5] = 'x'
    }

    if mode & 0o004 != 0 {
        chars[6] = 'r'
    }
    if mode & 0o002 != 0 {
        chars[7] = 'w'
    }
    if mode & 0o001 != 0 {
        chars[8] = 'x'
    }

    // Setuid, setgid, sticky bits
    if mode & 0o4000 != 0 {
        chars[2] = if chars[2] == 'x' { 's' } else { 'S' }
    }
    if mode & 0o2000 != 0 {
        chars[5] = if chars[5] == 'x' { 's' } else { 'S' }
    }
    if mode & 0o1000 != 0 {
        chars[8] = if chars[8] == 'x' { 't' } else { 'T' }
    }

    // Convert Vec<char> to String
    let output: String = chars.into_iter().collect();
    format!("{}{}", if metadata.is_dir() { 'd' } else { '-' }, output)
}
