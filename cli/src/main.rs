mod theme;

use std::error::Error;
use dotenv::dotenv;
use newsapi::{ NewsAPIResponse, NewsAPI, Endpoint, Country, Article };

async fn render_articles(articles: &Vec<Article>) {

    let theme = theme::default();

    theme.print_text("# Top headlines");

    for a in articles {
        theme.print_text(&format!("`{}`", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---")
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);

    // let articles = newsapi.fetch_async().await?;
    let articles = newsapi.fetch()?;
    
    render_articles(articles.articles()).await;

    Ok(())
}
