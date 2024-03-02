use indexmap::map::IndexMap; // we need IndexMap to have deterministic order of keys when `.iter()`
use std::collections::HashMap;

use crate::input::SizeOption;
use clap::crate_version;

/// Returns the name of the crate with the version in the `Cargo.toml`
pub fn format_version_to_display() -> String {
    format!("snakepipe@{}(rust)", crate_version!())
}

/// Takes in [`crate::input::InitOptions::features_with_version`] and extracts a HashMap with
/// - keys: versions
/// - values: vector of string of features
///
/// Example:
///
/// Input (as [`std::collections::HashMap<String, String>`])
/// ```json
/// {
///   "throttle": "snakepipe@1.0.0(node)",
///   "render": "snakepipe@1.0.0(rust)",
///   "gamestate": "snakepipe@1.0.0(rust)"
/// }
/// ```
///
/// Output (as [`indexmap::map::IndexMap<String, Vec<String>>`])
/// ```json
/// {
///   "snakepipe@1.0.0(rust)": [
///     "gamestate",
///     "render"
///   ],
///   "snakepipe@1.0.0(node)": [
///     "throttle"
///   ]
/// }
/// ```
pub fn extract_versions_with_features(
    features_with_version: HashMap<String, String>,
) -> IndexMap<String, Vec<String>> {
    let mut versions_with_features: IndexMap<String, Vec<String>> = IndexMap::new();
    features_with_version.iter().for_each(|(feature, version)| {
        if versions_with_features.contains_key(version) {
            versions_with_features
                .entry(version.to_string())
                .and_modify(|features| features.push(feature.to_string()));
        } else {
            versions_with_features.insert(version.to_string(), vec![feature.to_string()]);
        }
    });
    versions_with_features.values_mut().for_each(|features| {
        features.sort();
    });
    return versions_with_features;
}

/// Takes in the output of [`extract_versions_with_features`] and formats it in a string
///
/// Example:
///
/// Input
/// ```json
/// {
///   "snakepipe@1.0.0(rust)": [
///     "gamestate",
///     "render"
///   ],
///   "snakepipe@1.0.0(node)": [
///     "throttle"
///   ]
/// }
/// ```
///
/// Ouput
///
/// `snakepipe@1.0.0(rust): gamestate/render - snakepipe@1.0.0(node): throttle`
pub fn format_version_with_features(
    versions_with_features: IndexMap<String, Vec<String>>,
) -> String {
    if versions_with_features.len() == 1 {
        if let Some((version, _)) = versions_with_features.iter().next() {
            return version.to_string();
        }
        return "Unknown version".to_string();
    }
    let couple_version_features: Vec<String> = versions_with_features
        .iter()
        .map(|(version, features)| format!("{}: {}", version, features.join("/")))
        .collect();
    return couple_version_features.join(" - ");
}

/// Takes in [`crate::input::InitOptions::features_with_version`] and formats it to a string
///
/// Composes [extract_versions_with_features] and [format_version_with_features].
pub fn format_version(features_with_version: HashMap<String, String>) -> String {
    return format_version_with_features(extract_versions_with_features(features_with_version));
}

pub fn format_metadatas(
    metadatas: HashMap<String, String>,
    _frame_duration: u32,
    _size: SizeOption,
) -> String {
    let mut result: Vec<String> = Vec::new();
    if let Some(value) = metadatas.get("throttled") {
        if value == "on" {
            result.push("Record mode".to_string());
        }
    }
    if let Some(value) = metadatas.get("render-browser-host") {
        result.push(format!("Mirrored on {}", value));
    }
    return result.join(" / ");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn should_have_only_one_key_with_all_features_if_the_same_version_evrywhere() {
        let mut version = HashMap::new();
        version.insert("gamestate".to_string(), "snakepipe@1.0.0(rust)".to_string());
        version.insert("throttle".to_string(), "snakepipe@1.0.0(rust)".to_string());
        version.insert("render".to_string(), "snakepipe@1.0.0(rust)".to_string());
        let mut result = IndexMap::new();
        result.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec![
                "gamestate".to_string(),
                "render".to_string(),
                "throttle".to_string(),
            ],
        );
        assert_eq!(extract_versions_with_features(version), result);
    }

    #[test]
    fn should_extract_all_existing_versions() {
        let mut version = HashMap::new();
        version.insert("gamestate".to_string(), "snakepipe@1.0.0(rust)".to_string());
        version.insert("throttle".to_string(), "snakepipe@1.0.0(node)".to_string());
        version.insert("render".to_string(), "snakepipe@1.0.0(rust)".to_string());
        let mut result = IndexMap::new();
        result.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec!["gamestate".to_string(), "render".to_string()],
        );
        result.insert(
            "snakepipe@1.0.0(node)".to_string(),
            vec!["throttle".to_string()],
        );
        assert_eq!(extract_versions_with_features(version), result);
    }

    #[test]
    fn should_not_show_features_if_they_all_have_the_same_version() {
        let mut versions_with_features = IndexMap::new();
        versions_with_features.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec![
                "gamestate".to_string(),
                "render".to_string(),
                "throttle".to_string(),
            ],
        );
        let expected = "snakepipe@1.0.0(rust)";
        assert_eq!(
            format_version_with_features(versions_with_features),
            expected.to_string()
        );
    }

    #[test]
    fn should_show_features_if_version_differs() {
        let mut versions_with_features = IndexMap::new();
        versions_with_features.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec!["gamestate".to_string(), "render".to_string()],
        );
        versions_with_features.insert(
            "snakepipe@1.0.0(node)".to_string(),
            vec!["throttle".to_string()],
        );
        let expected = "snakepipe@1.0.0(rust): gamestate/render - snakepipe@1.0.0(node): throttle";
        assert_eq!(
            format_version_with_features(versions_with_features),
            expected.to_string()
        );
    }
}
