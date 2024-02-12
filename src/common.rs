use std::collections::HashMap;

use clap::crate_version;

pub fn format_version_to_display() -> String {
    format!("snakepipe@{}(rust)", crate_version!())
}

pub fn reduce_version_to_display(
    features_with_version: HashMap<String, String>,
) -> HashMap<String, Vec<String>> {
    let mut versions_with_features: HashMap<String, Vec<String>> = HashMap::new();
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

// pub fn display_version(features_with_version: HashMap<String, String>) -> String {
//     let versions_with_features = reduce_version_to_display(features_with_version);
// }

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
        let mut result = HashMap::new();
        result.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec![
                "gamestate".to_string(),
                "render".to_string(),
                "throttle".to_string(),
            ],
        );
        assert_eq!(reduce_version_to_display(version), result);
    }

    #[test]
    fn should_have_as_many_versions_as_it_exist() {
        let mut version = HashMap::new();
        version.insert("gamestate".to_string(), "snakepipe@1.0.0(rust)".to_string());
        version.insert("throttle".to_string(), "snakepipe@1.0.0(node)".to_string());
        version.insert("render".to_string(), "snakepipe@1.0.0(rust)".to_string());
        let mut result = HashMap::new();
        result.insert(
            "snakepipe@1.0.0(rust)".to_string(),
            vec!["gamestate".to_string(), "render".to_string()],
        );
        result.insert(
            "snakepipe@1.0.0(node)".to_string(),
            vec!["throttle".to_string()],
        );
        assert_eq!(reduce_version_to_display(version), result);
    }
}
