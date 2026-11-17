//! The AllCraft Studio and project links (Help menu, About dialog, top-bar Studio button).

/// The app's slug on GitHub.
pub const APP: &str = "lightcraft";
/// The AllCraft community discussions.
pub const DISCORD: &str = "https://github.com/thepixopik/lightcraft/discussions";
/// The AllCraft Studio website.
pub const WEBSITE: &str = "https://the-allcraft.pages.dev";
/// This app's live path on The AllCraft Studio.
pub const APP_PAGE: &str = "https://the-allcraft.pages.dev/light/";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/thepixopik/lightcraft";

/// The user documentation (docs/ in the repository).
pub const HELP: &str = "https://github.com/thepixopik/lightcraft/tree/main/docs";

pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "The AllCraft Light Help", HELP),
    ("app.discord", "AllCraft Community…", DISCORD),
    ("app.website", "The AllCraft Light Web", APP_PAGE),
    ("app.github", "LightCraft on GitHub", GITHUB),
    ("app.artcraft", "The AllCraft Studio", WEBSITE),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

/// Where feedback and bug reports go.
pub const FEEDBACK: &str = "https://github.com/thepixopik/lightcraft/issues/new";

/// The URL behind a link command id.
pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_https_and_name_this_app() {
        for (id, label, url) in LINKS {
            assert!(url.starts_with("https://"), "{id}");
            assert!(!label.is_empty());
            assert_eq!(url_of(id), Some(*url));
        }
        assert!(APP_PAGE.starts_with(WEBSITE));
        assert!(GITHUB.ends_with(&format!("/thepixopik/{APP}")));
    }
}
