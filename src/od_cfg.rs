use super::errors;
use std::{collections, fs, path};

pub struct OverdropConf {
    dirs: Vec<path::PathBuf>,
}

impl OverdropConf {
    pub fn new(basedirs: &[path::PathBuf], name: &str, version: Option<u32>) -> Self {
        let mut dirs = Vec::with_capacity(basedirs.len());
        let name_d = name.to_owned() + ".d";
        let ver = version.unwrap_or(0);
        for bdir in basedirs {
            let mut dpath = path::PathBuf::from(bdir);
            dpath.push(name_d.clone());
            dirs.push(dpath);
            if ver > 0 {
                dirs.push(format!("v{}", ver).to_owned().into());
            }
        }
        Self { dirs }
    }

    pub fn scan_unique_files(
        &self,
    ) -> errors::Result<collections::BTreeMap<String, path::PathBuf>> {
        let mut files_map = collections::BTreeMap::new();
        for dir in &self.dirs {
            let dir_iter = match fs::read_dir(dir) {
                Ok(iter) => iter,
                _ => continue,
            };
            for dir_entry in dir_iter {
                let entry = match dir_entry {
                    Ok(f) => f,
                    _ => continue,
                };
                let fpath = entry.path();
                let fname = entry.file_name().into_string().unwrap();

                // Ignore dotfiles.
                if fname.starts_with('.') {
                    continue;
                };
                // Ignore non-TOML.
                if !fname.ends_with(".toml") {
                    continue;
                };

                // Check filetype, ignore non-file.
                let meta = match entry.metadata() {
                    Ok(m) => m,
                    _ => continue,
                };
                if !meta.file_type().is_file() {
                    if let Ok(target) = fs::read_link(&fpath) {
                        // A devnull symlink is a special case to ignore previous file-names.
                        if target == path::PathBuf::from("/dev/null") {
                            trace!("Nulled config file '{}'", fpath.display());
                            files_map.remove(&fname);
                        }
                    }
                    continue;
                }

                // TODO(lucab): return something smarter than a PathBuf.
                trace!("Found config file '{}' at '{}'", fname, fpath.display());
                files_map.insert(fname, fpath);
            }
        }
        Ok(files_map)
    }
}
