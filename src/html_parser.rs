use scraper::{ElementRef, Html, Selector, error::SelectorErrorKind};

// Parseia um html e retorna um vetor contendo os elementos atingidos pelo seletor css passado como parâmetro

pub fn parse_html<'document, 'selector>(
    document: &'document Html,
    selector: &'selector str,
) -> Result<Vec<ElementRef<'document>>, SelectorErrorKind<'selector>> {
    let parsed_selector = Selector::parse(selector)?;
    let elements = document.select(&parsed_selector).collect();

    Ok(elements)
}
