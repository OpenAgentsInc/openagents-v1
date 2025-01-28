use anyhow::{bail, Result};
use clap::Parser;
use dotenvy::dotenv;
use openagents::{
    repo::{cleanup_temp_dir, clone_repository, RepoContext},
    repomap::generate_repo_map,
    server::services::{deepseek::DeepSeekService, github_issue::GitHubService, StreamUpdate},
};
use std::env;
use std::io::{stdout, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// GitHub issue number to solve
    #[arg(short, long)]
    issue: u32,

    /// GitHub repository (format: owner/name)
    #[arg(short, long, default_value = "OpenAgentsInc/openagents")]
    repo: String,

    /// Execute changes on GitHub (create branch, post comments, create PR)
    #[arg(long)]
    live: bool,
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
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN not found in environment or .env file"))?;

    // Parse repo owner and name
    let repo_parts: Vec<&str> = cli.repo.split('/').collect();
    if repo_parts.len() != 2 {
        bail!("Invalid repository format. Expected 'owner/name'");
    }
    let (owner, repo_name) = (repo_parts[0], repo_parts[1]);

    // Initialize services
    let github_service = GitHubService::new(Some(github_token.clone()))?;
    let deepseek_service = DeepSeekService::new(api_key.clone());

    // Fetch issue details
    print_colored("\nFetching issue details...\n", Color::Blue)?;
    let issue = github_service
        .get_issue(owner, repo_name, cli.issue)
        .await?;
    
    // Define the temporary directory path
    let temp_dir = env::temp_dir().join(format!("solver_{}", cli.issue));

    // Clean up any existing temp directory first
    cleanup_temp_dir(&temp_dir);

    // Create the temporary directory
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create temporary directory: {}", e))?;
    println!("Temporary directory created at: {:?}", temp_dir);

    // Create context
    let ctx = RepoContext::new(temp_dir.clone(), api_key, Some(github_token));

    // Clone the repository
    let repo_url = format!("https://github.com/{}/{}", owner, repo_name);
    let _repo = clone_repository(&repo_url, &ctx.temp_dir)?;

    // Generate repository map
    print_colored("\nGenerating repository map...\n", Color::Blue)?;
    let map = generate_repo_map(&ctx.temp_dir);

    // Create a new branch for the solution (if in live mode)
    let branch_name = format!("solver/issue-{}", cli.issue);
    if cli.live {
        print_colored(&format!("\nCreating branch '{}'...\n", branch_name), Color::Blue)?;
        // TODO: Implement branch creation
    } else {
        print_colored(
            &format!("\n[DRY RUN] Would create branch '{}'\n", branch_name),
            Color::Yellow,
        )?;
    }
    
    // Analyze issue and generate implementation plan
    let plan_prompt = format!(
        "You are a Rust development expert. Analyze this GitHub issue and repository map to create an implementation plan.\n\n\
        Issue #{}: {}\n{}\n\nRepository map:\n{}\n\n\
        Create a detailed implementation plan including:\n\
        1. Files that need to be created or modified\n\
        2. Key functionality to implement\n\
        3. Required dependencies or imports\n\
        4. Testing strategy\n\
        Be specific and focus on practical implementation details.",
        issue.number, issue.title, issue.body, map
    );

    print_colored("\nGenerating Implementation Plan:\n", Color::Yellow)?;
    let mut implementation_plan = String::new();
    let mut in_reasoning = true;
    let mut stream = deepseek_service.chat_stream(plan_prompt, true).await;

    while let Some(update) = stream.recv().await {
        match update {
            StreamUpdate::Reasoning(r) => {
                print_colored(&r, Color::Yellow)?;
            }
            StreamUpdate::Content(c) => {
                if in_reasoning {
                    println!();
                    print_colored("\nImplementation Plan:\n", Color::Green)?;
                    in_reasoning = false;
                }
                print!("{}", c);
                implementation_plan.push_str(&c);
                stdout().flush()?;
            }
            StreamUpdate::Done => break,
            _ => {}
        }
    }
    println!();

    // Post implementation plan as comment if in live mode
    if cli.live {
        print_colored("\nPosting implementation plan to GitHub...\n", Color::Blue)?;
        let comment = format!(
            "# Implementation Plan\n\n\
            Based on the analysis of the issue and codebase, here's the proposed implementation plan:\n\n\
            {}\n\n\
            I'll now proceed with implementing this solution.",
            implementation_plan
        );
        github_service
            .post_comment(owner, repo_name, cli.issue, &comment)
            .await?;
    } else {
        print_colored("\n[DRY RUN] Would post implementation plan to GitHub:\n", Color::Yellow)?;
        println!("{}", implementation_plan);
    }

    // TODO: Generate solution
    print_colored("\nGenerating solution...\n", Color::Blue)?;
    // Implementation will go here
    
    // TODO: Track file modifications
    let mut modified_files = Vec::new();
    
    if cli.live {
        print_colored("\nCreating pull request...\n", Color::Blue)?;
        // TODO: Implement PR creation
    } else {
        print_colored("\n[DRY RUN] Summary of changes that would be made:\n", Color::Yellow)?;
        if modified_files.is_empty() {
            println!("No files modified yet");
        } else {
            for file in modified_files {
                println!("- Would modify: {}", file);
            }
        }
    }

    // Clean up at the end
    cleanup_temp_dir(&temp_dir);

    Ok(())
}