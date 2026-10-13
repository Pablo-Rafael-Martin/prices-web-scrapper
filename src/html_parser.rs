use scraper::{Html, Selector, error::SelectorErrorKind};

pub fn parse_html<'selector>(html: &str, selector: &'selector str) -> Result<usize, SelectorErrorKind<'selector>> {
    let document = Html::parse_document(html);

    match Selector::parse(selector) {
        Ok(element) => {
            return Ok(document.select(&element).count());
        },
        Err(error) => {
            return Err(error);
        }
    };
}