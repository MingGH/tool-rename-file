use uuid::Uuid;
use std::fs;
use std::path::Path;

fn main() {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    let files = fs::read_dir(current_dir).expect("无法读取目录");

    for file in files {
        if let Ok(entry) = file {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                println!("file_name:{:?}", file_name);
                let file_name_str = file_name.to_string_lossy();
                let extension = match path.extension() {
                    Some(ext) => ext.to_string_lossy().to_string(),
                    None => String::new(),
                };

                let new_file_name = format!("{}.{}", Uuid::new_v4().to_string(), extension);
                let new_path = Path::new(&path).with_file_name(&new_file_name);
                println!("new_file_name:{}", new_file_name);
                println!("new_path:{:?}", new_path);

                println!();
                fs::rename(&path, &new_path).expect("无法重命名文件");
            }
        }
    }

}