use anyhow::{bail, Result};
use clap::Parser;
use dotenvy::dotenv;
use openagents::{
    repo::{cleanup_temp_dir, clone_repository, run_cargo_tests, RepoContext},
    repomap::generate_repo_map,
    server::services::{deepseek::DeepSeekService, github_issue::GitHubService, StreamUpdate},
};
use std::env;
use std::io::{stdout, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Generate test suggestions for uncovered functionality
    #[arg(long)]
    test: bool,
}

fn print_colored(text: &str, color: Color) -> Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(stdout, "{}", text)?;
    stdout.reset()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load .env file first
    if let Err(e) = dotenv() {
        bail!("Failed to load .env file: {}", e);
    }

    // Get API keys immediately and fail if not present
    let api_key = env::var("DEEPSEEK_API_KEY")
        .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY not found in environment or .env file"))?;
    let github_token = env::var("GITHUB_TOKEN").ok();

    // Define the temporary directory path
    let temp_dir = env::temp_dir().join("rust_app_temp");

    // Clean up any existing temp directory first
    cleanup_temp_dir(&temp_dir);

    // Create the temporary directory
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create temporary directory: {}", e))?;
    println!("Temporary directory created at: {:?}", temp_dir);

    // Create context
    let ctx = RepoContext::new(temp_dir.clone(), api_key, github_token.clone());

    // Clone the repository
    let repo_url = "https://github.com/OpenAgentsInc/openagents";
    let _repo = clone_repository(repo_url, &ctx.temp_dir)?;

    // Generate and store the repository map
    let map = generate_repo_map(&ctx.temp_dir);
    println!("{}", map);

    // Run cargo test
    let test_output = run_cargo_tests(&ctx.temp_dir).await?;

    // Initialize DeepSeek service
    let service = DeepSeekService::new(ctx.api_key);

    if cli.test {
        println!("\nAnalyzing test coverage and generating test suggestions...");

        // First, analyze the test output to find uncovered modules/functions
        let coverage_prompt = format!(
            "You are a Rust testing expert. Analyze this test output and repository map to identify \
            specific functions or modules that lack test coverage. Focus only on test coverage analysis.\n\n\
            Test output:\n{}\n\nRepository map:\n{}\n\n\
            List the specific functions/modules that need test coverage, in order of importance. \
            For each one, explain why it needs testing and what scenarios should be tested.",
            test_output, map
        );

        print_colored("\nTest Coverage Analysis Reasoning:\n", Color::Yellow)?;
        let mut coverage_analysis = String::new();
        let mut in_reasoning = true;
        let mut stream = service.chat_stream(coverage_prompt, true).await;

        while let Some(update) = stream.recv().await {
            match update {
                StreamUpdate::Reasoning(r) => {
                    print_colored(&r, Color::Yellow)?;
                }
                StreamUpdate::Content(c) => {
                    if in_reasoning {
                        println!();
                        print_colored("\nTest Coverage Analysis:\n", Color::Green)?;
                        in_reasoning = false;
                    }
                    print!("{}", c);
                    coverage_analysis.push_str(&c);
                    stdout().flush()?;
                }
                StreamUpdate::Done => break,
                _ => {}
            }
        }
        println!();

        // Then, generate a specific test implementation for the most important uncovered functionality
        let test_prompt = "Based on the coverage analysis above, write a complete Rust test implementation for the \
            most critical uncovered function/module. Include:\n\
            1. All necessary imports\n\
            2. Test module setup with #[cfg(test)]\n\
            3. Required test fixtures and mocks\n\
            4. Multiple test cases covering different scenarios\n\
            5. Comments explaining the test strategy\n\n\
            Write the complete test code that could be directly added to the appropriate test file. \
            Make sure to handle edge cases and error conditions.".to_string();

        print_colored("\nTest Implementation Reasoning:\n", Color::Yellow)?;
        let mut test_code = String::new();
        let mut in_reasoning = true;
        let mut stream = service.chat_stream(test_prompt, true).await;

        while let Some(update) = stream.recv().await {
            match update {
                StreamUpdate::Reasoning(r) => {
                    print_colored(&r, Color::Yellow)?;
                }
                StreamUpdate::Content(c) => {
                    if in_reasoning {
                        println!();
                        print_colored("\nSuggested Test Implementation:\n", Color::Green)?;
                        in_reasoning = false;
                    }
                    print!("{}", c);
                    test_code.push_str(&c);
                    stdout().flush()?;
                }
                StreamUpdate::Done => break,
                _ => {}
            }
        }
        println!();

        // Post the suggestions as a GitHub comment if token is available
        if let Some(token) = github_token {
            println!("\nPosting test suggestions to GitHub...");
            let github_service = GitHubService::new(Some(token))?;

            let comment = format!(
                "# Test Coverage Analysis\n\n\
                The following analysis identifies areas needing test coverage:\n\n\
                {}\n\n\
                # Suggested Test Implementation\n\n\
                Here's a proposed test implementation for the most critical area:\n\n\
                ```rust\n{}\n```\n\n\
                Please review and consider implementing these test cases to improve coverage.",
                coverage_analysis, test_code
            );

            github_service
                .post_comment("OpenAgentsInc", "openagents", 592, &comment)
                .await?;
            println!("Test suggestions posted to GitHub issue #592");
        } else {
            println!("\nSkipping GitHub comment - GITHUB_TOKEN not found");
        }
    } else {
        println!("\nRun with --test flag to generate test suggestions");
    }

    // Clean up at the end
    cleanup_temp_dir(&temp_dir);

    Ok(())
}
