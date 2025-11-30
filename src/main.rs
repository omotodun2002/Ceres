use clap::Parser;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;


use ceres::config::{Config, Command};
use ceres::clients::openai::OpenAIClient;
use ceres::storage::DatasetRepository;
use ceres::pg::DatasetRepository as PgRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Load environment variables from .env file
    dotenv().ok();

    /// Setup Logging (could be improved with more configuration options)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    /// Parse command line arguments (as above, could be improved)
    let config = Config::parse();

    /// Db connection
    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    /// Services
    let repo = DatasetRepository::new(pool);
    let openai_client = OpenAIClient::new(&config.openai_api_key);

    /// Commands
    match config.command {
        Command::Harvest { portal_url } => {
            info!("Starting harvest from portal: {}", portal_url);
            /// TODO: CkanClient usage to harvest datasets
            /// let ckan_client = CkanClient::new(&portal_url);
            /// ... harvesting logic, i'll do this later ...
            println!("Harvesting from {} is not yet implemented.", portal_url);
        }
        Command::Search { query, limit } => {
            info!("Searching for: '{}' (limit: {})", query, limit);
            

            /// Query conversion to embeddings
            let vector = openai_client.get_embeddings(&query).await?;

            /// optional but search implementation in repository
            /// let results = repo.search(vector, limit).await?;
            /// println!("Found {} results.", results.len());

            println!("Vector generated successfully (len: {}). Search implementation pending in repository.", vector.len());
        }

    }

    Ok(())
}


