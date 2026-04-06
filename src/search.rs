/*use tvdb::Tvdb;
//use tmdb::TmdbClient;
use crate::config::Config;
use crate::error::AppError;

/*pub fn search_tmdb(config: &Config, title: &str, year: Option<i32>, type_id: Option<String>) -> Result<(), AppError> {
    let tmdb_client = TmdbClient::new(&config.tmdb_api_key);
    // Implémente la logique de recherche dans TMDb
    Ok(())
}*/

pub async fn search_tvdb(config: &Config, title: &str, year: Option<i32>) -> Result<(), AppError> {
    let tvdb_client = Tvdb::new(&config.tvdb_api_key);

    // Recherche série
    if year.is_none() {
        let search = tvdb_client.search(title).await?;
        // Implémente la logique de recherche dans TVDb
        
    }else {
        let search = tvdb_client.search(title).year(year).await?;
        // Implémente la logique de recherche dans TVDb
        
    }
    
    // Implémente la logique de recherche dans TVDb
    Ok(())
}
*/