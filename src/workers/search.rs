use html_parser::{Dom, Element, Node};

use crate::models::{Media, BASE_URL, ENCONDING, UA};

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
        let dom = Dom::parse(&res)?;
        let movies = dom
            .children
            .into_iter()
            .map(|node| match node {
                html_parser::Node::Text(_) => None,
                html_parser::Node::Element(e) => Some(e),
                html_parser::Node::Comment(_) => None,
            })
            .flatten()
            .collect::<Vec<Element>>()
            .into_iter()
            .map(|element| {
                if element.classes.len() == 0 {
                    return None;
                }
                if element.classes[0] != "listItems" {
                    return None;
                }
                return Some(element.children);
            })
            .flatten()
            .collect::<Vec<Vec<Node>>>();
        println!("{:?}", movies);
        todo!()
    }
}
