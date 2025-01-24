use std::fs;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use git2::Repository;
use openagents::repomap::generate_repo_map;
use openagents::server::services::deepseek::{DeepSeekService, ChatMessage};
use anyhow::{Result, bail};
use dotenvy::dotenv;

fn cleanup_temp_dir(temp_dir: &std::path::PathBuf) {
    if temp_dir.exists() {
        if let Err(e) = fs::remove_dir_all(temp_dir) {
            eprintln!("Warning: Failed to clean up temporary directory: {}", e);
        } else {
            println!("Temporary directory removed.");
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file first
    if let Err(e) = dotenv() {
        bail!("Failed to load .env file: {}", e);
    }

    // Get API key immediately and fail if not present
    let api_key = env::var("DEEPSEEK_API_KEY")
        .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY not found in environment or .env file"))?;

    // Define the temporary directory path
    let temp_dir = env::temp_dir().join("rust_app_temp");

    // Clean up any existing temp directory first
    cleanup_temp_dir(&temp_dir);

    // Create the temporary directory
    fs::create_dir_all(&temp_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create temporary directory: {}", e))?;
    println!("Temporary directory created at: {:?}", temp_dir);

    // Use a closure to handle the main logic and ensure cleanup
    let result = (|| async {
        // Clone the OpenAgentsInc/openagents repository into the temporary directory
        let repo_url = "https://github.com/OpenAgentsInc/openagents";
        println!("Cloning repository: {}", repo_url);
        let _repo = Repository::clone(repo_url, &temp_dir)
            .map_err(|e| anyhow::anyhow!("Failed to clone repository: {}", e))?;
        println!("Repository cloned successfully into: {:?}", temp_dir);

        // Generate and store the repository map
        let map = generate_repo_map(&temp_dir);
        println!("Repository Map:\n{}", map);

        // Run cargo test in the cloned repository with streaming output and capture results
        println!("Running cargo test in the cloned repository...");
        let mut test_output = String::new();
        let mut child = Command::new("cargo")
            .current_dir(&temp_dir)
            .arg("test")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to start cargo test: {}", e))?;

        // Stream stdout in real-time and capture it
        let stdout = child.stdout.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stderr"))?;
        
        // Spawn a thread to handle stdout
        let stdout_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut output = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }
            output
        });

        // Spawn a thread to handle stderr
        let stderr_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut output = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }
            output
        });

        // Wait for the command to complete
        let status = child.wait()
            .map_err(|e| anyhow::anyhow!("Failed to wait for cargo test: {}", e))?;

        // Wait for output threads to finish and collect their output
        let stdout_output = stdout_thread.join()
            .map_err(|_| anyhow::anyhow!("Failed to join stdout thread"))?;
        let stderr_output = stderr_thread.join()
            .map_err(|_| anyhow::anyhow!("Failed to join stderr thread"))?;
        test_output.push_str(&stdout_output);
        test_output.push_str(&stderr_output);

        // Print final test status
        if status.success() {
            println!("\nTests completed successfully!");
        } else {
            println!("\nTests failed!");
        }

        // Initialize DeepSeek service
        let service = DeepSeekService::new(api_key);

        // Create analysis prompt
        let analysis_prompt = format!(
            "Analyze this repository structure and test results. Suggest improvements and next steps.\n\n\
            Repository Structure:\n{}\n\nTest Results:\n{}",
            map, test_output
        );

        println!("\nRequesting DeepSeek analysis...");

        // Use reasoning mode for better analysis
        let mut stream = service.chat_stream(analysis_prompt, true).await;
        
        use openagents::server::services::StreamUpdate;
        let mut in_reasoning = true;
        let mut stdout = std::io::stdout();

        println!("\nReasoning Process:");
        while let Some(update) = stream.recv().await {
            match update {
                StreamUpdate::Reasoning(r) => {
                    print!("{}", r);
                    stdout.flush().ok();
                }
                StreamUpdate::Content(c) => {
                    if in_reasoning {
                        println!("\n\nAnalysis & Recommendations:");
                        in_reasoning = false;
                    }
                    print!("{}", c);
                    stdout.flush().ok();
                }
                StreamUpdate::Done => break,
                _ => {}
            }
        }
        println!();

        Ok(())
    })().await;

    // Always clean up, regardless of success or failure
    cleanup_temp_dir(&temp_dir);

    // Return the result
    result
}