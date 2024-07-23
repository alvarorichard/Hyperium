use crate::{
    models::{
        common::{BASE_URL, ENCONDING, UA},
        Media,
    },
    workers::parser,
};

use super::Worker;

impl Worker {
    pub async fn search(&self, term: &str) -> anyhow::Result<Vec<Media>> {
        let res = self
            .client
            .get(format!("{}/pesquisar/{}", BASE_URL, term))
            .header("User-Agent", UA)
            .header("Accept-Encoding", ENCONDING)
            .send()
            .await?
            .text()
            .await?;
        let movies = parser::element_from_search(&res);
        println!("{:?}", movies);
        todo!()
    }
}
