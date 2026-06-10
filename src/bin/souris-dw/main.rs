use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json;

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
    json: bool,
    no_auto_update: bool,
) -> Result<()> {
    let mut builder = souris_dw::SourisDW::builder()
        .auto_update(!no_auto_update);

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

    builder = builder.embed_metadata(embed_metadata);
    builder = builder.embed_thumbnail(embed_thumbnail);
    builder = builder.embed_subtitles(embed_subtitles);

    let dw = builder.build().await?;

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
        println!("{}", serde_json::to_string(&serde_json::json!({
            "type": "complete",
            "success": result.success,
            "path": result.path,
            "size": result.size
        }))?);
    } else {
        if result.success {
            println!("Downloaded: {}", result.path.unwrap_or_default());
        } else {
            eprintln!("Download failed: {}", result.error.unwrap_or_default());
        }
    }

    Ok(())
}

async fn handle_info(url: &str, json: bool) -> Result<()> {
    let dw = souris_dw::SourisDW::builder()
        .auto_update(false)
        .build()
        .await?;

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
        .await?;

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
        .await?;

    if check {
        let status = dw.update_check().await?;
        if json {
            println!("{}", serde_json::to_string(&status)?);
        } else {
            for dep in &status {
                println!("{}: {} ({})", dep.name, dep.version.as_deref().unwrap_or("not installed"), dep.path);
            }
        }
        return Ok(());
    }

    let status = dw.update().await?;

    if json {
        println!("{}", serde_json::to_string(&status)?);
    } else {
        for dep in &status {
            println!("{}: updated to {}", dep.name, dep.version.as_deref().unwrap_or("unknown"));
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
                    println!("{}", serde_json::to_string(&serde_json::json!({
                        "key": key,
                        "value": value
                    }))?);
                } else {
                    println!("{}", value);
                }
            } else {
                return Err(souris_dw::SourisError::ConfigError(format!("Unknown key: {}", key)).into());
            }
        }
        ConfigAction::Set { key, value } => {
            config.set(&key, &value)?;
            if json {
                println!("{}", serde_json::to_string(&serde_json::json!({
                    "key": key,
                    "value": value,
                    "status": "ok"
                }))?);
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
        .await?;

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
                    println!("{}: updated to {}", dep.name, dep.version.as_deref().unwrap_or("unknown"));
                }
            }
        }
    }

    Ok(())
}

async fn handle_tui() -> Result<()> {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;
    use souris_dw::tui::{app::AppState, events, ui};

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();
    let tick_rate = std::time::Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        match events::poll_event(tick_rate)? {
            Some(events::AppEvent::Key(key)) => {
                if let Some(action) = events::handle_key_event(key) {
                    match action {
                        events::Action::Quit => break,
                        events::Action::AddUrl => app.start_input(),
                        events::Action::Search => app.toggle_search(),
                        events::Action::Help => app.toggle_help(),
                        events::Action::Settings => app.toggle_settings(),
                        events::Action::MoveDown => app.move_selection_down(),
                        events::Action::MoveUp => app.move_selection_up(),
                        events::Action::MoveFirst => app.selected_index = 0,
                        events::Action::MoveLast => {
                            app.selected_index = app.downloads.len().saturating_sub(1)
                        }
                        events::Action::Confirm => {
                            if !app.input_buffer.is_empty() {
                                let url = app.input_buffer.clone();
                                app.add_download(url.clone(), url.clone(), "Unknown".to_string());
                                app.cancel_input();
                            }
                        }
                        events::Action::Cancel => app.cancel_input(),
                        _ => {}
                    }
                }
            }
            Some(events::AppEvent::Tick) => {}
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
