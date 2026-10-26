#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use anyhow::Context;
use anyhow::{anyhow, Result};
use log::{trace, Level};
use remove_dir_all::remove_dir_all;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process;
use warp_args::Args;

use crate::extractor::get_args;

mod executor;
mod extractor;

static STATIC_SUBFOLDER_NAME: &str = "static";
static PAYLOAD_ID_FILE_NAME: &str = ".warp-payload";
static LOCK_FILE_NAME: &str = ".lock";

fn cache_path(target: &str) -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new()
        .ok_or_else(|| anyhow!("No base directory found"))?;

    Ok(base_dirs.data_local_dir()
        .join("warp")
        .join("packages")
        .join(target))
}

fn is_current(cache_path: &Path, payload_id: &str) -> bool {
    fs::read_to_string(cache_path.join(PAYLOAD_ID_FILE_NAME)).is_ok_and(|id| id == payload_id)
}

fn extract(exe_path: &Path, cache_path: &Path, payload_id: &str) -> Result<()> {
    let mut tmp_name = cache_path.file_name().unwrap().to_owned();
    tmp_name.push(".tmp");
    let tmp_path = cache_path.with_file_name(tmp_name);

    if tmp_path.exists() {
        remove_dir_all(&tmp_path)
            .with_context(|| format!("Failed to remove directory {}", tmp_path.display()))?;
    }
    extractor::extract_to(exe_path, &tmp_path).with_context(|| {
        format!(
            "Failed to extract {} to {}",
            exe_path.display(),
            tmp_path.display()
        )
    })?;
    fs::write(tmp_path.join(PAYLOAD_ID_FILE_NAME), payload_id)?;

    if cache_path.exists() {
        remove_dir_all(cache_path)
            .with_context(|| format!("Failed to remove directory {}", cache_path.display()))?;
    }
    fs::rename(&tmp_path, cache_path).with_context(|| {
        format!(
            "Failed to move {} to {}",
            tmp_path.display(),
            cache_path.display()
        )
    })
}

fn create_cache_folder(self_path: &Path, args: &Args) -> Result<PathBuf> {
    let prefix = args
        .prefix
        .as_deref()
        .unwrap_or_else(|| Path::new(self_path.file_name().unwrap()));
    let cache_path = cache_path(&prefix.to_string_lossy())?;

    trace!("self_path={:?}", self_path);
    trace!("prefix={:?}", prefix);
    trace!("cache_path={:?}", cache_path);

    if cache_path.exists() && !fs::metadata(&cache_path)?.is_dir() {
        return Err(anyhow!("cache at {:?} is not a directory", &cache_path));
    } else {
        fs::create_dir_all(&cache_path)?;
    }

    Ok(cache_path)
}

fn clean_cache(cache_folder: &Path, uid: &str) -> Result<()> {
    let uid = OsStr::new(uid);
    let static_folder = OsStr::new(STATIC_SUBFOLDER_NAME);
    let lock_file = OsStr::new(LOCK_FILE_NAME);

    trace!("cleaning cache");
    for entry in fs::read_dir(cache_folder)? {
        let entry = entry?;

        if entry.file_name() == static_folder {
            trace!("skipped static subfolder {:?}", static_folder);
            continue;
        }

        if entry.file_name() == uid {
            trace!("skipped own uid {:?}", uid);
            continue;
        }

        if entry.file_name() == lock_file {
            continue;
        }

        trace!("removing entry {:?}", entry);
        if let Err(err) = remove_dir_all(entry.path()) {
            eprintln!(
                "Error while attempting to remove directory {:?}: {err}",
                entry.path()
            );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    if env::var("WARP_TRACE").is_ok() {
        simple_logger::init_with_level(Level::Trace)?;
    }

    let self_path = env::current_exe()?;
    let args = get_args(&self_path).unwrap();
    trace!("args = {:?}", args);

    let cache_folder = create_cache_folder(&self_path, &args)?;
    let subfolder = args.uid.as_deref().unwrap_or(STATIC_SUBFOLDER_NAME);
    let cache_path = cache_folder.join(subfolder);

    let target_path = cache_path.join(&args.target_file_name);

    trace!("target_exec={:?}", args.target_file_name);
    trace!("target_path={:?}", target_path);

    let clean_uid = args.uid.as_deref().filter(|_| args.clean);
    if clean_uid.is_some() || !is_current(&cache_path, &args.payload_id) {
        let lock = File::create(cache_folder.join(LOCK_FILE_NAME))?;
        lock.lock()?;

        if let Some(uid) = clean_uid {
            clean_cache(&cache_folder, uid)?;
        }
        if !is_current(&cache_path, &args.payload_id) {
            trace!("cache missing or outdated, extracting");
            extract(&self_path, &cache_path, &args.payload_id)?;
        }
    }

    process::exit(executor::execute(&target_path, args.hidden)?);
}
