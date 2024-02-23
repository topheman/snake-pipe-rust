/**
 *
 * @param {Array<"score" | "version">} features
 * @returns
 */
export function makeRenderInfos(features) {
  /**
   *
   * @param {import("../types").InitOptions} initOptions
   * @param {import("../types").Game} frameInfos
   * @param {HTMLElement} infosNode
   */
  return function renderInfos(initOptions, frameInfos, infosNode) {
    const infos = [
      features.includes("score") ? `<li>Score: ${frameInfos.score} - ${frameInfos.state}</li>` : false,
      features.includes("version") ? `<li>${makeVersion(initOptions.featuresWithVersion)}</li>` : false,
    ].filter(Boolean);
    infosNode.innerHTML = infos.join('');
  }
}

/**
 * Same implementation as in `src/common.rs`
 * @param {Record<string, string>} featuresWithVersion
 */
export function makeVersion(featuresWithVersion) {
  const versionsWithFeatures = Object.entries(featuresWithVersion).reduce((acc, [feature, version]) => {
    let tmp;
    if (acc[version]) {
      acc[version].push(feature);
    }
    else {
      acc[version] = [feature];
    }
    return acc;
  }, {});
  console.log("versionsWithFeatures", versionsWithFeatures, versionsWithFeatures.size);
  if (Object.keys(versionsWithFeatures).length === 1) {
    return Object.keys(versionsWithFeatures)[0];
  }
  return Object.entries(versionsWithFeatures).map(([version, features]) => `${version}: ${features.join('/')}`).join(' - ');
}
