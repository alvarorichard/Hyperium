use crate::models::Media;
use ego_tree::NodeRef;
use scraper::{Html, Node, Selector};
pub enum ParseError {
    NotFound,
    ParseError,
}
pub fn element_from_search(body: &str) -> Result<Vec<Media>, ParseError> {
    let doc = Html::parse_document(body);
    let selector = Selector::parse(".listItems").unwrap();
    let div = doc.select(&selector).next().ok_or(ParseError::NotFound)?;
    if !div.has_children() {
        return Ok(vec![]);
    }
    let media = div
        .children()
        .filter_map(|e| parse_media(e))
        .collect::<Vec<Media>>();
    Ok(media)
}

fn parse_media(element: NodeRef<Node>) -> Option<Media> {
    let element = element.value().to_owned();
    println!("{:?}", element);
    todo!()
}
