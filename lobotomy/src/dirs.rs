use std::path::PathBuf;

use directories::ProjectDirs;
use flexi_logger::FileSpec;

fn default_project_dirs() -> ProjectDirs {
    ProjectDirs::from("dev", "oakchris1955", "lobotomy").unwrap()
}

fn logs_dir() -> PathBuf {
    let mut local_data_dir = default_project_dirs().data_local_dir().to_path_buf();
    local_data_dir.push("logs");

    local_data_dir
}

pub fn log_filespec() -> FileSpec {
    FileSpec::default()
        .directory(logs_dir())
        .suppress_basename()
        .use_timestamp(true)
}
