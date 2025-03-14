use clap::Parser;
use dotenv::dotenv;
use git2::{Repository, StatusOptions};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::process::Command;

#[derive(Parser)]
#[clap(about = "Generate git commit messages using Claude API based on git changes")]
struct Args {
    /// Print verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentBlock {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set in environment or .env file");
    let args = Args::parse();
    
    // Try to open the repository at the current directory
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_) => {
            eprintln!("Error: Not in a git repository");
            std::process::exit(1);
        }
    };

    // Get git status
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_options))?;
    
    if statuses.is_empty() {
        println!("No changes to commit");
        return Ok(());
    }

    // Get the diff
    let diff = get_git_diff(args.verbose)?;
    
    if diff.is_empty() {
        println!("No staged changes to commit");
        return Ok(());
    }
    
    if args.verbose {
        println!("Sending the following diff to Claude:\n{}", diff);
    }
    
    // Generate commit message using Claude API
    let commit_message = get_claude_commit_message(&api_key, &diff)?;
    
    println!("Suggested commit message:\n\n{}", commit_message);
    
    // Ask for confirmation
    println!("Do you want to create a commit with this message? [y/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        // Create commit with the generated message
        let status = Command::new("git")
            .args(["commit", "-m", &commit_message])
            .status()?;
            
        if status.success() {
            println!("Commit created successfully!");
        } else {
            eprintln!("Failed to create commit");
        }
    }
    
    Ok(())
}

fn get_git_diff(verbose: bool) -> Result<String, Box<dyn Error>> {
    // Get staged changes
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()?;
    
    if !output.status.success() {
        return Err("Failed to execute git diff".into());
    }
    
    let mut diff = String::from_utf8(output.stdout)?;
    
    // If no staged changes, get unstaged changes
    if diff.is_empty() {
        let output = Command::new("git")
            .args(["diff"])
            .output()?;
        
        if !output.status.success() {
            return Err("Failed to execute git diff".into());
        }
        
        diff = String::from_utf8(output.stdout)?;
    }
    
    if verbose {
        println!("Got diff of length: {}", diff.len());
    }
    
    Ok(diff)
}

fn get_claude_commit_message(api_key: &str, diff: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    
    let system_message = "You are a helpful assistant that generates concise git commit messages based on diffs. \
                         Your commit messages should follow these guidelines:\
                         1. Start with a verb in the imperative mood (Add, Fix, Update, etc.)\
                         2. Be concise but descriptive (under 50 characters if possible)\
                         3. Focus on WHY the change is being made, not just WHAT\
                         4. If needed, provide more details in a separate paragraph after a blank line\
                         5. Format as a single line for the summary, then a blank line, then details if needed\
                         6. No need to add lists of files changed, as git already tracks that\
                         7. Return only the commit message text with no extra commentary";
    
    let user_message = format!("Generate a commit message for the following git diff:\n\n```\n{}\n```", diff);
    
    let request = AnthropicRequest {
        model: "claude-3-7-sonnet-20250219".to_string(),
        max_tokens: 300,
        system: system_message.to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
    };
    
    let response = client.post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()?;
    
    if !response.status().is_success() {
        let error_text = response.text()?;
        return Err(format!("API request failed: {}", error_text).into());
    }
    
    let response_data: AnthropicResponse = response.json()?;
    
    // Get text from the first content block
    if let Some(content_block) = response_data.content.first() {
        Ok(content_block.text.trim().to_string())
    } else {
        Err("No content received from Claude API".into())
    }
}