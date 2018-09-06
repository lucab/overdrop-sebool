//! Main CLI entrypoint and helpers.

use super::errors;
use super::od_cfg;
use clap;
use selinur;
use std::{collections, fs, io, path};
use toml;

use std::io::Read;

pub(crate) fn run() -> errors::Result<()> {
    let app_name = "overdrop-sebool";
    let _cli_top = clap::App::new(app_name)
        .version("0.0.1-dev")
        .author("Luca Bruno")
        .about("SELinux boolean runtime setting")
        .get_matches_safe()?;

    let basedirs = vec![
        path::PathBuf::from("/lib"),
        path::PathBuf::from("/run"),
        path::PathBuf::from("/etc"),
    ];

    let od = od_cfg::OverdropConf::new(&basedirs, app_name, None);
    let files = od.scan_unique_files()?;
    let cfg = merge_cfg(&files)?;
    apply_sebools(&cfg, true)?;

    Ok(())
}

pub(crate) fn merge_cfg(
    map: &collections::BTreeMap<String, path::PathBuf>,
) -> errors::Result<SeboolCfg> {
    let bools = collections::BTreeMap::new();
    let mut cfg = SeboolCfg { bools };
    for fpath in map.values() {
        let fp = fs::File::open(fpath)?;
        let mut bufrd = io::BufReader::new(fp);
        let mut buf = Vec::new();
        bufrd.read_to_end(&mut buf)?;
        let cfg_snip: SeboolCfgSnippet = toml::from_slice(&buf)?;
        cfg = accumulate_snippet(cfg, cfg_snip);
    }
    Ok(cfg)
}

pub(crate) fn accumulate_snippet(cur: SeboolCfg, snip: SeboolCfgSnippet) -> SeboolCfg {
    let mut res = cur;
    for (k, v) in snip.bools {
        // This mimics an `Option<bool>`, not supported by TOML syntax.
        if v == "" {
            res.bools.remove(&k);
            continue;
        }

        if let Ok(b) = v.parse() {
            res.bools.insert(k, b);
        };
    }
    res
}

pub(crate) fn apply_sebools(map: &SeboolCfg, commit: bool) -> errors::Result<()> {
    let sefs = selinur::sys::SELinuxFs::open_path(selinur::sys::DEFAULT_PATH)?;
    for (sebool_k, sebool_v) in &map.bools {
        sefs.set_sebool_pending(sebool_k, *sebool_v)?;
        info!("SELinux boolean pending - {}: {}", sebool_k, sebool_v);
    }
    if !map.bools.is_empty() && commit {
        sefs.commit_pending_bools()?;
        info!("Committed all pending SELinux booleans");
    }
    Ok(())
}

pub(crate) struct SeboolCfg {
    bools: collections::BTreeMap<String, bool>,
}

#[derive(Deserialize)]
pub(crate) struct SeboolCfgSnippet {
    #[serde(rename = "sebool")]
    bools: collections::BTreeMap<String, String>,
}
