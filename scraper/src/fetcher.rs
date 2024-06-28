use crate::extractor::ExtractedWebsite;
use fantoccini::Locator;
use std::thread::sleep;
use std::time::Duration;

pub async fn fetch_website(
    extracted_website: ExtractedWebsite,
    client: &fantoccini::Client,
) -> Result<String, ()> {
    client
        .goto(&extracted_website.url)
        .await
        .expect("could't navigate to the website");
    client
        .wait()
        .at_most(Duration::from_secs(5))
        .for_element(Locator::Css("body"))
        .await
        .expect("could't find body");
    sleep(Duration::from_secs(2));
    let html = client
        .find(Locator::Css("html"))
        .await
        .expect("could't find html")
        .html(false)
        .await
        .expect("could't retrieve the HTML contents of this element.");
    let screenshot = client
        .screenshot()
        .await
        .expect("could't take a screenshot");
    let filename = format!("screenshots/{}.png", extracted_website.id);
    // Save the screenshot to a file
    std::fs::write(filename, &screenshot).expect("could't write file");
    Ok(html)
}
