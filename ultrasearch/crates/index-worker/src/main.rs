//! Short-lived worker process for content indexing.
//!
//! Minimal first cut: extract a single file to text using the content-extractor
//! stack, honoring size/char limits and an optional Extractous backend toggle.

use anyhow::{Context, Result};
use clap::Parser;
use content_extractor::{ExtractContext, ExtractorStack};
use core_types::DocKey;
use dotenvy::dotenv;
use std::path::PathBuf;
use std::{env, fs};
use tracing::{info, warn};

/// Basic single-file extraction job (temporary until full job contract lands).
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Volume id for the document key.
    #[arg(long)]
    volume_id: u16,
    /// File reference number for the document key.
    #[arg(long)]
    file_id: u64,
    /// Path to the file to extract.
    #[arg(long)]
    path: PathBuf,
    /// Maximum bytes to read.
    #[arg(long, default_value = "10485760")] // 10 MiB
    max_bytes: usize,
    /// Maximum characters to keep.
    #[arg(long, default_value = "100000")] // 100k chars
    max_chars: usize,
    /// Enable Extractous backend (requires feature extractous_backend).
    #[arg(long, default_value = "false")]
    enable_extractous: bool,
}

fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut args = Args::parse();

    // Allow env override for Extractous toggle to align with service config later.
    if let Ok(val) = env::var("ULTRASEARCH_ENABLE_EXTRACTOUS") {
        args.enable_extractous = matches!(val.as_str(), "1" | "true" | "TRUE");
    }

    let doc_key = DocKey::from_parts(args.volume_id, args.file_id);
    let ext_owned = args
        .path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());
    let ctx = ExtractContext {
        path: args
            .path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("path is not valid UTF-8"))?,
        max_bytes: args.max_bytes,
        max_chars: args.max_chars,
        ext_hint: ext_owned.as_deref(),
        mime_hint: None,
    };

    // Ensure file exists before spinning extractors.
    fs::metadata(&args.path)
        .with_context(|| format!("file missing or unreadable: {}", args.path.display()))?;

    let stack = ExtractorStack::with_extractous_enabled(args.enable_extractous);
    info!(
        "extracting {:?} with extractous_enabled={}",
        args.path, args.enable_extractous
    );

    match stack.extract(doc_key, &ctx) {
        Ok(out) => {
            let preview = out.text.chars().take(200).collect::<String>();
            info!(
                "extracted bytes={}, truncated={}, lang={:?}",
                out.bytes_processed, out.truncated, out.lang
            );
            println!("{preview}");
        }
        Err(err) => {
            warn!("extraction failed: {err}");
            return Err(err);
        }
    }

    Ok(())
}
