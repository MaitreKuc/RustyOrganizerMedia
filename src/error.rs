//Gére les erreurs de l'application de manière centralisée

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erreur IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Erreur SQLite: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("Erreur API : {0}")]
    Api(#[from] reqwest::Error),
}