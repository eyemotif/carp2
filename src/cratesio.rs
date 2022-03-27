use crate::cargoreader::CrateInfo;
use crate::utils::Result;
use crates_index::{Crate, Index};
use semver::{Version, VersionReq};

fn get_crate_version<'a>(crte: &'a Crate) -> Result<Version> {
    let crate_latest: Version = crte
        .highest_stable_version()
        .ok_or(format!(
            "Could not find the latest version for crate '{}'.",
            crte.name()
        ))?
        .version()
        .parse()?;
    Ok(crate_latest)
}

fn compare_crate_version(current_version: &VersionReq, crte: &Crate) -> Result<bool> {
    let crate_latest = get_crate_version(crte)?;
    Ok(current_version.matches(&crate_latest))
}

fn compare_crate_version_strict(current_version: &Version, crte: &Crate) -> Result<bool> {
    let crate_latest = get_crate_version(crte)?;
    Ok(*current_version == crate_latest)
}

pub fn out_of_date_crate_infos<'a>(
    strict: bool,
    index: &Index,
    crate_infos: &'a [CrateInfo],
) -> Result<Vec<&'a CrateInfo>> {
    let crate_infos_vec: Vec<_> = crate_infos.iter().collect();
    let crate_compares: Result<Vec<bool>> = crate_infos
        .iter()
        .map(|crate_info| {
            let crate_from_crate_info = index
                .crate_(&crate_info.name)
                .ok_or(format!("Crate '{}' not found.", &crate_info.name))?;
            if strict {
                compare_crate_version_strict(&crate_info.version, &crate_from_crate_info)
            } else {
                compare_crate_version(&crate_info.version_req, &crate_from_crate_info)
            }
        })
        .collect();
    let mut filtered_crate_infos: Vec<&CrateInfo> = vec![];

    for (compare, crate_info) in crate_compares?.iter().zip(crate_infos_vec) {
        if !*compare {
            filtered_crate_infos.push(crate_info)
        }
    }
    Ok(filtered_crate_infos)
}
