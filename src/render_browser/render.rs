use crate::common::format_version_to_display;
use crate::input::parse_gamestate;
use crate::render_browser::common::UrlToDisplay;
use crate::render_browser::server::launch_server;

pub fn run(port: u16) {
    match parse_gamestate() {
        Ok(input) => {
            let mut options_passthrough = input.options.clone();
            options_passthrough
                .features_with_version
                .insert("render-browser".to_string(), format_version_to_display());
            let url_to_display = UrlToDisplay::new(port);
            options_passthrough.metadatas.insert(
                "render-browser-host".to_string(),
                format!("{}", url_to_display.url).to_string(),
            );
            println!("{}\r", serde_json::to_string(&options_passthrough).unwrap());
            let _ = launch_server(input.lines, options_passthrough, port);
        }
        Err(e) => {
            eprintln!("Error occurred while parsing stdin: \"{}\"", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
