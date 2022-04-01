use crate::dependency::{get_versions_from_str, Dependency};
use crate::utils::Result;
use crates_index::{Crate, Index};
use semver::{Version, VersionReq};

fn get_crate_latest_version(crte: &Crate) -> Result<Version> {
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
    Ok(current_version.matches(&get_crate_latest_version(crte)?))
}

fn compare_crate_version_strict(current_version: &Version, crte: &Crate) -> Result<bool> {
    Ok(current_version == &get_crate_latest_version(crte)?)
}

pub fn get_crate_latest_versions(crte: &Crate) -> Result<(VersionReq, Option<Version>)> {
    let crate_latest_str = crte
        .highest_stable_version()
        .ok_or(format!(
            "Could not find the latest version for crate '{}'.",
            crte.name()
        ))?
        .version();
    let versions = get_versions_from_str(crate_latest_str)?;
    Ok(versions)
}

pub fn out_of_date_dependencies(
    strict: bool,
    only_strict: bool,
    index: &Index,
    dependencies: Vec<Dependency>,
) -> Result<Vec<(Dependency, Version)>> {
    let crate_compares: Result<Vec<_>> = dependencies
        .iter()
        .map(|dependency| {
            let crate_from_dependency = index
                .crate_(&dependency.name)
                .ok_or(format!("Crate '{}' not found.", &dependency.name))?;
            if only_strict {
                compare_crate_version_strict(
                    dependency.version.as_ref().ok_or(format!(
                        "Dependency version for '{}' is not specific enough to compare strictly.",
                        &dependency.name
                    ))?,
                    &crate_from_dependency,
                )
                .map(|comp| (comp, crate_from_dependency))
            } else if strict {
                if let Some(version) = &dependency.version {
                    compare_crate_version_strict(version, &crate_from_dependency)
                        .map(|comp| (comp, crate_from_dependency))
                } else {
                    compare_crate_version(&dependency.version_req, &crate_from_dependency)
                        .map(|comp| (comp, crate_from_dependency))
                }
            } else {
                compare_crate_version(&dependency.version_req, &crate_from_dependency)
                    .map(|comp| (comp, crate_from_dependency))
            }
        })
        .collect();
    dependencies
        .into_iter()
        .zip(crate_compares?)
        .filter_map(
            |(dependency, (compare, crte))| match get_crate_latest_version(&crte) {
                Ok(latest_version) => {
                    if !compare {
                        Some(Ok((dependency, latest_version)))
                    } else {
                        None
                    }
                }
                Err(err) => Some(Err(err)),
            },
        )
        .collect()
}

pub fn crate_has_version(version: &VersionReq, crte: &Crate) -> Result<bool> {
    for crate_version in crte.versions() {
        let crate_semver: Version = crate_version.version().parse()?;
        if version.matches(&crate_semver) {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn get_index() -> Result<Index> {
    let index = Index::new_cargo_default()?;
    Ok(index)
}
