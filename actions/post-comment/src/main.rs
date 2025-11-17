use octocrab::Octocrab;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let token: String = beta::input("token")?;
    let number: u64 = beta::input("number")?;
    let body: String = beta::input("body")?;
    let repo = beta::repository()?;

    let client = Octocrab::builder().personal_token(token).build()?;

    let comment = client
        .issues(repo.owner, repo.name)
        .create_comment(number, &body)
        .await?;

    beta::output("comment-url", comment.html_url.to_string());

    Ok(())
}
