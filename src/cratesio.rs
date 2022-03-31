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

pub fn out_of_date_dependencies<'a>(
    strict: bool,
    only_strict: bool,
    index: &Index,
    dependencies: &'a Vec<&'a Dependency>,
) -> Result<Vec<(&'a Dependency, Version)>> {
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
    let mut filtered_dependencies: Vec<_> = vec![];

    for ((compare, crte), dependency) in crate_compares?.iter().zip(dependencies) {
        if !compare {
            filtered_dependencies.push((*dependency, get_crate_latest_version(&crte)?))
        }
    }
    Ok(filtered_dependencies)
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
