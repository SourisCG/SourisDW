use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "souris-dw")]
#[command(version = "0.1.0")]
#[command(about = "Cross-platform music & video downloader for YouTube and Spotify", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    json: bool,

    #[arg(long, global = true)]
    quiet: bool,

    #[arg(long, global = true)]
    no_auto_update: bool,

    #[arg(long, global = true)]
    no_color: bool,

    #[arg(long, global = true)]
    timeout: Option<u64>,

    #[arg(long, global = true)]
    max_retries: Option<u32>,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        url: String,
        #[arg(short, long)]
        format: Option<String>,
        #[arg(short, long)]
        quality: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long)]
        parallel: Option<usize>,
        #[arg(long)]
        embed_metadata: bool,
        #[arg(long)]
        embed_thumbnail: bool,
        #[arg(long)]
        embed_subtitles: bool,
        #[arg(long)]
        audio_only: bool,
        #[arg(long)]
        video_only: bool,
        #[arg(long)]
        cookies: Option<String>,
        #[arg(long)]
        cookies_from_browser: Option<String>,
    },
    Info {
        url: String,
    },
    Search {
        query: String,
        #[arg(short, long)]
        platform: Option<String>,
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    Update {
        #[arg(long)]
        yt_dlp: bool,
        #[arg(long)]
        ffmpeg: bool,
        #[arg(long)]
        self_: bool,
        #[arg(long)]
        check: bool,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Deps {
        #[command(subcommand)]
        action: DepsAction,
    },
    Uninstall {
        #[arg(long)]
        keep_config: bool,
    },
    Tui,
}

#[derive(Subcommand)]
enum ConfigAction {
    Get { key: String },
    Set { key: String, value: String },
    Show,
}

#[derive(Subcommand)]
enum DepsAction {
    Status,
    Update,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Download {
            url,
            format,
            quality,
            output,
            parallel,
            embed_metadata,
            embed_thumbnail,
            embed_subtitles,
            audio_only,
            video_only,
            cookies,
            cookies_from_browser,
        } => {
            handle_download(
                &url,
                format.as_deref(),
                quality.as_deref(),
                output.as_deref(),
                parallel,
                embed_metadata,
                embed_thumbnail,
                embed_subtitles,
                audio_only,
                video_only,
                cookies.as_deref(),
                cookies_from_browser.as_deref(),
                cli.json,
                cli.no_auto_update,
            )
            .await
        }
        Commands::Info { url } => handle_info(&url, cli.json).await,
        Commands::Search {
            query,
            platform,
            limit,
        } => handle_search(&query, platform.as_deref(), limit, cli.json).await,
        Commands::Update {
            yt_dlp,
            ffmpeg,
            self_,
            check,
        } => handle_update(yt_dlp, ffmpeg, self_, check, cli.json).await,
        Commands::Config { action } => handle_config(action, cli.json).await,
        Commands::Deps { action } => handle_deps(action, cli.json).await,
        Commands::Uninstall { keep_config } => handle_uninstall(keep_config, cli.json).await,
        Commands::Tui => handle_tui().await,
    };

    if let Err(e) = result {
        if cli.json {
            let error = serde_json::json!({
                "type": "error",
                "code": "GENERAL_ERROR",
                "message": e.to_string()
            });
            println!("{}", serde_json::to_string(&error).unwrap());
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_download(
    url: &str,
    format: Option<&str>,
    quality: Option<&str>,
    output: Option<&str>,
    parallel: Option<usize>,
    embed_metadata: bool,
    embed_thumbnail: bool,
    embed_subtitles: bool,
    audio_only: bool,
    video_only: bool,
    cookies: Option<&str>,
    cookies_from_browser: Option<&str>,
    json: bool,
    no_auto_update: bool,
) -> Result<()> {
    let mut builder = souris_dw::SourisDW::builder().auto_update(!no_auto_update);

    if let Some(f) = format {
        builder = builder.format_str(f)?;
    }

    if let Some(q) = quality {
        builder = builder.quality_str(q)?;
    }

    if let Some(o) = output {
        builder = builder.output(o);
    }

    if let Some(p) = parallel {
        builder = builder.parallel(p);
    }

    if let Some(c) = cookies {
        builder = builder.cookies_file(c);
    }

    if let Some(c) = cookies_from_browser {
        builder = builder.cookies_from_browser(c);
    }

    builder = builder.embed_metadata(embed_metadata);
    builder = builder.embed_thumbnail(embed_thumbnail);
    builder = builder.embed_subtitles(embed_subtitles);

    let dw = builder.build().await;

    let mut req = if audio_only {
        dw.download_audio(url)
    } else if video_only {
        dw.download_video(url)
    } else {
        dw.download(url)
    };

    if let Some(f) = format {
        req = req.format_str(f)?;
    }

    if let Some(q) = quality {
        req = req.quality_str(q)?;
    }

    let result = req.run().await?;

    if json {
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "type": "complete",
                "success": result.success,
                "path": result.path,
                "size": result.size
            }))?
        );
    } else if result.success {
        println!("Downloaded: {}", result.path.unwrap_or_default());
    } else {
        eprintln!("Download failed: {}", result.error.unwrap_or_default());
    }

    Ok(())
}

async fn handle_info(url: &str, json: bool) -> Result<()> {
    let dw = souris_dw::SourisDW::builder()
        .auto_update(false)
        .build()
        .await;

    let info = dw.info(url).await?;

    if json {
        println!("{}", serde_json::to_string(&info)?);
    } else {
        println!("Title: {}", info.title);
        println!("Platform: {}", info.platform);
        println!("ID: {}", info.id);
        if let Some(duration) = info.duration {
            println!("Duration: {}s", duration);
        }
        if let Some(ref uploader) = info.uploader {
            println!("Uploader: {}", uploader);
        }
    }

    Ok(())
}

async fn handle_search(
    query: &str,
    _platform: Option<&str>,
    limit: usize,
    json: bool,
) -> Result<()> {
    let dw = souris_dw::SourisDW::builder()
        .auto_update(false)
        .build()
        .await;

    let results = dw.search(query).await?;

    if json {
        println!("{}", serde_json::to_string(&results)?);
    } else {
        for (i, item) in results.iter().take(limit).enumerate() {
            println!("{}. {} [{}]", i + 1, item.title, item.platform);
            println!("   URL: {}", item.url);
            if let Some(duration) = item.duration {
                println!("   Duration: {}s", duration);
            }
            println!();
        }
    }

    Ok(())
}

async fn handle_update(
    _yt_dlp: bool,
    _ffmpeg: bool,
    _self_: bool,
    check: bool,
    json: bool,
) -> Result<()> {
    let dw = souris_dw::SourisDW::builder()
        .auto_update(false)
        .build()
        .await;

    if check {
        let status = dw.update_check().await?;
        if json {
            println!("{}", serde_json::to_string(&status)?);
        } else {
            for dep in &status {
                println!(
                    "{}: {} ({})",
                    dep.name,
                    dep.version.as_deref().unwrap_or("not installed"),
                    dep.path
                );
            }
        }
        return Ok(());
    }

    let status = dw.update().await?;

    if json {
        println!("{}", serde_json::to_string(&status)?);
    } else {
        for dep in &status {
            println!(
                "{}: updated to {}",
                dep.name,
                dep.version.as_deref().unwrap_or("unknown")
            );
        }
    }

    Ok(())
}

async fn handle_config(action: ConfigAction, json: bool) -> Result<()> {
    let mut config = souris_dw::AppConfig::load()?;

    match action {
        ConfigAction::Get { key } => {
            if let Some(value) = config.get(&key) {
                if json {
                    println!(
                        "{}",
                        serde_json::to_string(&serde_json::json!({
                            "key": key,
                            "value": value
                        }))?
                    );
                } else {
                    println!("{}", value);
                }
            } else {
                return Err(
                    souris_dw::SourisError::ConfigError(format!("Unknown key: {}", key)).into(),
                );
            }
        }
        ConfigAction::Set { key, value } => {
            config.set(&key, &value)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string(&serde_json::json!({
                        "key": key,
                        "value": value,
                        "status": "ok"
                    }))?
                );
            } else {
                println!("Set {} = {}", key, value);
            }
        }
        ConfigAction::Show => {
            if json {
                println!("{}", serde_json::to_string(&config)?);
            } else {
                let contents = toml::to_string_pretty(&config)
                    .map_err(|e| souris_dw::SourisError::ConfigError(e.to_string()))?;
                println!("{}", contents);
            }
        }
    }

    Ok(())
}

async fn handle_deps(action: DepsAction, json: bool) -> Result<()> {
    let dw = souris_dw::SourisDW::builder()
        .auto_update(false)
        .build()
        .await;

    match action {
        DepsAction::Status => {
            let status = dw.update_check().await?;
            if json {
                println!("{}", serde_json::to_string(&status)?);
            } else {
                for dep in &status {
                    let status_icon = if dep.installed { "[x]" } else { "[ ]" };
                    println!(
                        "{} {} {} ({})",
                        status_icon,
                        dep.name,
                        dep.version.as_deref().unwrap_or("not installed"),
                        dep.path
                    );
                }
            }
        }
        DepsAction::Update => {
            let status = dw.update().await?;
            if json {
                println!("{}", serde_json::to_string(&status)?);
            } else {
                for dep in &status {
                    println!(
                        "{}: updated to {}",
                        dep.name,
                        dep.version.as_deref().unwrap_or("unknown")
                    );
                }
            }
        }
    }

    Ok(())
}

async fn handle_uninstall(keep_config: bool, json: bool) -> Result<()> {
    let exe_path = std::env::current_exe()?;

    if json {
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "type": "uninstall",
                "binary": exe_path.display().to_string(),
                "keep_config": keep_config
            }))?
        );
    } else {
        println!("SourisDW Uninstaller");
        println!("===================");
        println!();
        println!("Binary: {}", exe_path.display());

        if !keep_config {
            if let Some(config_dir) = directories::ProjectDirs::from("", "", "souris-dw") {
                let config_path = config_dir.config_dir();
                let data_path = config_dir.data_dir();
                println!("Config: {}", config_path.display());
                println!("Data:   {}", data_path.display());
            }
        } else {
            println!("Keeping config and data files.");
        }
        println!();
    }

    fs_err::remove_file(&exe_path).map_err(|e| {
        souris_dw::SourisError::ConfigError(format!(
            "Failed to remove binary: {}. You may need to run with sudo.",
            e
        ))
    })?;

    if !keep_config {
        if let Some(config_dir) = directories::ProjectDirs::from("", "", "souris-dw") {
            let _ = fs_err::remove_dir_all(config_dir.config_dir());
            let _ = fs_err::remove_dir_all(config_dir.data_dir());
        }
    }

    if !json {
        println!("Uninstalled successfully.");
    }

    Ok(())
}

#[allow(dead_code)]
enum TuiEvent {
    DownloadStarted {
        index: usize,
        title: String,
        platform: String,
    },
    DownloadProgress {
        index: usize,
        percent: f64,
        speed: String,
        eta: String,
    },
    DownloadComplete {
        index: usize,
        path: String,
        size: u64,
    },
    DownloadError {
        index: usize,
        message: String,
    },
    SearchResults(Vec<souris_dw::core::types::SearchItem>),
    SearchError(String),
}

async fn handle_tui() -> Result<()> {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;
    use souris_dw::tui::{app::{AppState, SETTINGS_OPTIONS}, events, ui};

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();
    let tick_rate = std::time::Duration::from_millis(100);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TuiEvent>();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        match events::poll_event(tick_rate)? {
            Some(events::AppEvent::Key(key)) => {
                let action = events::handle_key_event(key, &app.input_mode);
                if let Some(action) = action {
                    match action {
                        events::Action::Back => {
                            if app.show_error_popup {
                                app.clear_error();
                                app.waiting_for_quit = false;
                            } else if app.show_help {
                                app.show_help = false;
                                app.waiting_for_quit = false;
                            } else if app.show_settings {
                                app.show_settings = false;
                                app.waiting_for_quit = false;
                            } else if app.show_search {
                                app.toggle_search();
                                app.waiting_for_quit = false;
                            } else {
                                match app.input_mode {
                                    souris_dw::tui::app::InputMode::Input => {
                                        app.cancel_input();
                                    }
                                    souris_dw::tui::app::InputMode::Search => {
                                        app.cancel_input();
                                    }
                                    souris_dw::tui::app::InputMode::Normal => {
                                        if app.waiting_for_quit {
                                            break;
                                        } else {
                                            app.waiting_for_quit = true;
                                            app.status_message =
                                                Some("Press Esc again to quit".into());
                                        }
                                    }
                                }
                            }
                        }
                        events::Action::ForceQuit => break,
                        events::Action::AddUrl => app.start_input(),
                        events::Action::Search => app.toggle_search(),
                        events::Action::Help => app.toggle_help(),
                        events::Action::Settings => app.toggle_settings(),
                        events::Action::CopyUrl => {
                            if app.copy_selected_url() {
                                app.status_message = Some("URL copied to clipboard".into());
                            } else {
                                app.status_message = Some("Failed to copy URL".into());
                            }
                        }
                        events::Action::CopyError => {
                            if app.copy_error(app.selected_index) {
                                app.status_message = Some("Copied".into());
                            } else if app.copy_selected_url() {
                                app.status_message = Some("URL copied".into());
                            } else {
                                app.status_message = Some("Nothing to copy".into());
                            }
                            app.waiting_for_quit = false;
                        }
                        events::Action::MoveDown => {
                            if app.show_settings {
                                if app.settings_index < SETTINGS_OPTIONS.len().saturating_sub(1) {
                                    app.settings_index += 1;
                                }
                            } else if app.show_search {
                                if app.search_index < app.search_results.len().saturating_sub(1) {
                                    app.search_index += 1;
                                }
                            } else {
                                app.move_selection_down();
                            }
                            app.waiting_for_quit = false;
                        }
                        events::Action::MoveUp => {
                            if app.show_settings {
                                if app.settings_index > 0 {
                                    app.settings_index -= 1;
                                }
                            } else if app.show_search {
                                if app.search_index > 0 {
                                    app.search_index -= 1;
                                }
                            } else {
                                app.move_selection_up();
                            }
                            app.waiting_for_quit = false;
                        }
                        events::Action::MoveFirst => {
                            if app.show_settings {
                                app.settings_index = 0;
                            } else {
                                app.selected_index = 0;
                            }
                            app.waiting_for_quit = false;
                        }
                        events::Action::MoveLast => {
                            if app.show_settings {
                                app.settings_index = SETTINGS_OPTIONS.len().saturating_sub(1);
                            } else {
                                app.selected_index = app.downloads.len().saturating_sub(1);
                            }
                            app.waiting_for_quit = false;
                        }
                        events::Action::Confirm => {
                            if app.show_settings {
                                app.cycle_setting_value(app.settings_index);
                            } else if app.show_search && !app.search_results.is_empty() {
                                if let Some(result) = app.search_results.get(app.search_index).cloned() {
                                    let title = result.title.clone();
                                    let url = result.url.clone();
                                    let platform = result.platform.clone();
                                    let index = app.add_download(url.clone(), title, platform.clone());
                                    app.toggle_search();
                                    app.status_message = Some("Starting download...".into());

                                    let tx = tx.clone();
                                    let audio_only = app.config.audio_only;
                                    let audio_format = app.config.audio_format.clone();
                                    let default_format = app.config.default_format.clone();
                                    let default_quality = app.config.default_quality.clone();
                                    let output_dir = app.config.output_dir.clone();
                                    let embed_metadata = app.config.embed_metadata;
                                    let embed_thumbnail = app.config.embed_thumbnail;
                                    let embed_subtitles = false;

                                    tokio::spawn(async move {
                                        let result = async {
                                            let builder = souris_dw::SourisDW::builder()
                                                .auto_update(true)
                                                .yt_dlp_channel("stable")
                                                .spotify_credentials(
                                                    std::env::var("SOURIS_SPOTIFY_CLIENT_ID")
                                                        .unwrap_or_default(),
                                                    std::env::var("SOURIS_SPOTIFY_CLIENT_SECRET")
                                                        .unwrap_or_default(),
                                                );
                                            let dw = builder.build().await;

                                            let _ = tx.send(TuiEvent::DownloadStarted {
                                                index,
                                                title: url.clone(),
                                                platform: platform.clone(),
                                            });

                                            let mut req = if audio_only {
                                                dw.download_audio(&url)
                                            } else {
                                                dw.download(&url)
                                            };

                                            req = req
                                                .output(output_dir)
                                                .embed_metadata(embed_metadata)
                                                .embed_thumbnail(embed_thumbnail)
                                                .embed_subtitles(embed_subtitles);

                                            if audio_only {
                                                req = req.format_str(&audio_format)?;
                                            } else {
                                                req = req.format_str(&default_format)?;
                                                req = req.quality_str(&default_quality)?;
                                            }

                                            req.run().await
                                        }
                                        .await;

                                        match result {
                                            Ok(res) => {
                                                let _ = tx.send(TuiEvent::DownloadComplete {
                                                    index,
                                                    path: res.path.unwrap_or_default(),
                                                    size: res.size.unwrap_or(0),
                                                });
                                            }
                                            Err(e) => {
                                                let _ = tx.send(TuiEvent::DownloadError {
                                                    index,
                                                    message: e.to_string(),
                                                });
                                            }
                                        }
                                    });
                                }
                            } else if app.show_search && !app.input_buffer.is_empty() {
                                let query = app.input_buffer.clone();
                                app.status_message = Some(format!("Searching: {}", query));

                                let tx = tx.clone();
                                let input_buf = app.input_buffer.clone();
                                app.input_buffer.clear();
                                app.search_results.clear();

                                tokio::spawn(async move {
                                    let result = async {
                                        let builder = souris_dw::SourisDW::builder()
                                            .auto_update(true)
                                            .build().await;
                                        builder.search(&input_buf).await
                                    }
                                    .await;

                                    match result {
                                        Ok(results) => {
                                            let _ = tx.send(TuiEvent::SearchResults(results));
                                        }
                                        Err(e) => {
                                            let _ = tx.send(TuiEvent::SearchError(e.to_string()));
                                        }
                                    }
                                });
                            } else if !app.input_buffer.is_empty() {
                                let url = app.input_buffer.clone();
                                let index = app.add_download(
                                    url.clone(),
                                    url.clone(),
                                    "Unknown".to_string(),
                                );
                                app.cancel_input();
                                app.status_message = Some("Starting download...".into());

                                let tx = tx.clone();
                                let audio_only = app.config.audio_only;
                                let audio_format = app.config.audio_format.clone();
                                let default_format = app.config.default_format.clone();
                                let default_quality = app.config.default_quality.clone();
                                let output_dir = app.config.output_dir.clone();
                                let embed_metadata = app.config.embed_metadata;
                                let embed_thumbnail = app.config.embed_thumbnail;
                                let embed_subtitles = false;

                                tokio::spawn(async move {
                                    let result = async {
                                        let builder = souris_dw::SourisDW::builder()
                                            .auto_update(true)
                                            .yt_dlp_channel("stable")
                                            .spotify_credentials(
                                                std::env::var("SOURIS_SPOTIFY_CLIENT_ID")
                                                    .unwrap_or_default(),
                                                std::env::var("SOURIS_SPOTIFY_CLIENT_SECRET")
                                                    .unwrap_or_default(),
                                            );
                                        let dw = builder.build().await;

                                        let _ = tx.send(TuiEvent::DownloadStarted {
                                            index,
                                            title: url.clone(),
                                            platform: "YouTube".into(),
                                        });

                                        let mut req = if audio_only {
                                            dw.download_audio(&url)
                                        } else {
                                            dw.download(&url)
                                        };

                                        req = req
                                            .output(output_dir)
                                            .embed_metadata(embed_metadata)
                                            .embed_thumbnail(embed_thumbnail)
                                            .embed_subtitles(embed_subtitles);

                                        if audio_only {
                                            req = req.format_str(&audio_format)?;
                                        } else {
                                            req = req.format_str(&default_format)?;
                                            req = req.quality_str(&default_quality)?;
                                        }

                                        req.run().await
                                    }
                                    .await;

                                    match result {
                                        Ok(res) => {
                                            let _ = tx.send(TuiEvent::DownloadComplete {
                                                index,
                                                path: res.path.unwrap_or_default(),
                                                size: res.size.unwrap_or(0),
                                            });
                                        }
                                        Err(e) => {
                                            let _ = tx.send(TuiEvent::DownloadError {
                                                index,
                                                message: e.to_string(),
                                            });
                                        }
                                    }
                                });
                            }
                        }
                        events::Action::Delete => {
                            if matches!(
                                app.input_mode,
                                souris_dw::tui::app::InputMode::Input
                                    | souris_dw::tui::app::InputMode::Search
                            ) {
                                app.input_buffer.pop();
                            }
                        }
                        events::Action::Pause => {}
                        events::Action::Cancel => {
                            app.cancel_input();
                            app.waiting_for_quit = false;
                        }
                        events::Action::CharInput(c) => {
                            app.input_buffer.push(c);
                            app.waiting_for_quit = false;
                        }
                        events::Action::DeleteChar => {
                            app.input_buffer.pop();
                        }
                        events::Action::FocusNext | events::Action::FocusPrev => {
                            app.waiting_for_quit = false;
                        }
                    }
                }
            }
            Some(events::AppEvent::Tick) => {
                while let Ok(event) = rx.try_recv() {
                    match event {
                        TuiEvent::DownloadStarted {
                            index,
                            title,
                            platform,
                        } => {
                            if let Some(dl) = app.downloads.get_mut(index) {
                                dl.title = title;
                                dl.platform = platform;
                                dl.status = souris_dw::tui::app::DownloadStatus::Downloading;
                            }
                        }
                        TuiEvent::DownloadProgress {
                            index,
                            percent,
                            speed,
                            eta,
                        } => {
                            if let Some(dl) = app.downloads.get_mut(index) {
                                dl.progress = percent;
                                dl.speed = speed;
                                dl.eta = eta;
                                dl.status = souris_dw::tui::app::DownloadStatus::Downloading;
                            }
                        }
                        TuiEvent::DownloadComplete { index, path, size } => {
                            if let Some(dl) = app.downloads.get_mut(index) {
                                dl.status = souris_dw::tui::app::DownloadStatus::Complete;
                                dl.progress = 100.0;
                                dl.path = Some(path);
                                dl.size = Some(size);
                            }
                            app.status_message = Some("Download complete".into());
                        }
                        TuiEvent::DownloadError { index, message } => {
                            if let Some(dl) = app.downloads.get_mut(index) {
                                dl.status = souris_dw::tui::app::DownloadStatus::Error(message);
                            }
                        }
                        TuiEvent::SearchResults(results) => {
                            app.search_results = results.into_iter().map(|r| {
                                souris_dw::tui::app::SearchResult {
                                    title: r.title,
                                    url: r.url,
                                    platform: r.platform,
                                    duration: r.duration,
                                    selected: false,
                                }
                            }).collect();
                            app.search_index = 0;
                            app.status_message = Some(format!("Found {} results", app.search_results.len()));
                        }
                        TuiEvent::SearchError(msg) => {
                            app.status_message = Some(format!("Search failed: {}", msg));
                        }
                    }
                }
            }
            Some(events::AppEvent::Quit) => break,
            None => {}
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
