use crate::config::CONFIG;
use crate::fs::save_to_json;
use crate::processor::UrlIDPair;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;
use std::path::Path;
use std::{fs, io};
use tracing::{debug, error};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedWebsite {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub tags: Vec<usize>,
}
// -----
// - [Velociraptor](https://github.com/Velocidex/velociraptor)
// - [FlareVM](https://github.com/mandiant/flare-vm)
// - [Volatility](https://github.com/volatilityfoundation/volatility)
// - [Autopsy](https://www.autopsy.com/)
// - [Zimmermans's Tools](https://ericzimmerman.github.io/#!index.md)
// - [Nirsoft Tools](https://www.nirsoft.ne
//
// TODO: Add md path
// TODO: handle [notlink](https)
fn extract_titles_and_urls(content: &str) -> Vec<(&str, &str)> {
    let mut result: Vec<(&str, &str)> = Vec::new();

    let lines: Vec<&str> = content.lines().collect();

    for mut line in lines {
        while line.len() != 0 {
            if let Some(url_block_name_start) = line.find("[") {
                let mut title = line[..url_block_name_start].trim();
                if let Some(url_block_name_end) = line.find("](") {
                    title = &line[url_block_name_start + 2..url_block_name_end].trim();
                }
                if let Some(title_start) = line.find(".") {
                    if url_block_name_start > title_start {
                        title = &line[title_start + 1..url_block_name_start].trim();
                        if title.len() > 0 {
                            if let Some(char) = title.chars().last() {
                                if char == '-' {
                                    title = title[..title.len() - 1].trim();
                                }
                            }
                        }
                    }
                }
                if let Some(url_block_url_start) = line.find("](") {
                    if let Some(url_block_url_relative_end) = line[url_block_url_start..].find(")")
                    {
                        let url_block_url_end = url_block_url_relative_end + url_block_url_start;
                        let url = line[url_block_url_start + 2..url_block_url_end].trim();
                        result.push((title, url));
                        line = &line[url_block_url_end + 1..];
                    } else {
                        line = ""
                    }
                } else {
                    line = ""
                }
            } else {
                line = ""
            }
        }
    }
    result
}

fn read_md_file(path: &Path) -> io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

// fn read_downloaded_websites(path: &str) -> io::Result<Vec<ExtractedWebsite>> {
//     let file = fs::File::open(path)?;
//     let mut reader = io::BufReader::new(file);
//     let mut content = String::new();
//     reader.read_to_string(&mut content)?;
//     let websites: Vec<ExtractedWebsite> = serde_json::from_str(&content)?;
//     Ok(websites)
// }

pub async fn extract_links(
    folder_path: &str,
    mut downloaded_websites: Vec<UrlIDPair>,
) -> std::io::Result<Vec<ExtractedWebsite>> {
    let tags = extract_tags(folder_path).await.unwrap();
    let mut extracted_links: Vec<ExtractedWebsite> = Vec::new();
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "md" {
                    if let Ok(markdown_content) = read_md_file(&entry.path()) {
                        let websites = extract_titles_and_urls(&markdown_content);
                        for (title, url) in websites {
                            let id = if let Some(downloaded_website) =
                                downloaded_websites.iter().find(|pair| pair.url == url)
                            {
                                downloaded_website.id
                            } else {
                                if let Some(max_id) =
                                    downloaded_websites.iter().map(|link| link.id).max()
                                {
                                    max_id + 1
                                } else {
                                    0
                                }
                            };
                            downloaded_websites.push(UrlIDPair {
                                id: id.clone(),
                                url: url.to_string(),
                            });
                            debug!("extracted website {id} {url}");
                            let tag_indexes: Vec<usize> = entry
                                .path()
                                .to_str()
                                .unwrap()
                                .replace(&['.', '/'][..], "")
                                .split("\\")
                                .filter_map(|&ref item| tags.iter().position(|tag| tag == item))
                                .collect();
                            extracted_links.push(ExtractedWebsite {
                                id,
                                title: title.to_owned(),
                                url: url.to_owned(),
                                tags: tag_indexes,
                            });
                        }
                    } else {
                        error!("Failed to read file: {}", &entry.path().display());
                    }
                }
            }
        }
    }
    Ok(extracted_links)
}

pub async fn extract_tags(folder_path: &str) -> std::io::Result<HashSet<String>> {
    let mut tags: HashSet<String> = HashSet::new();
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        println!("{}",entry.path().to_str().unwrap());
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "md" {
                    let path = entry.path().to_str().unwrap().replace(&['.', '/'][..], "");
                    let mut tags_from_string: Vec<&str> = path.split("\\").collect();
                    tags_from_string.pop();
                    println!("{:?}",tags_from_string);
                    tags_from_string.into_iter().for_each(|tag| {
                        if tag != "NTHW" {
                            tags.insert(tag.to_owned());
                        }
                    });
                }
            }
        }
    }
    save_to_json(
        &CONFIG.nthw_frontend_tags_path,
        tags.iter().cloned().collect(),
    )
    .await
    .unwrap();
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use crate::extractor::extract_titles_and_urls;

    #[test]
    fn extracts_every_link() {
        const CONTENT: &str = r#"# Not The Hidden Wiki

        🎓 Akademia NTHW - Lista szkoleń którymi warto się zainteresować (tylko po Polsku)
        -----
        
        1. Szkoła Security: Security Starter - [link](https://l.szkolasecurity.pl/starter/) 86. Szkoła Security2: Security Starter2 - [link](https://l.szkolasecurity.pl/starter2/)
        2. Szkoła Security: Testowanie Bezpieczeństwa Web Aplikacji Masters - [link](https://szkolasecurity.pl/tbwam/)
        3. Szkoła Security: Security Analyst Krok Pierwszy - [link](https://szkolasecurity.pl/blue1/)
        4. Szkoła Security: Python Dla Bezpieczników - [link](https://szkolasecurity.pl/python/)
        5. Szkoła Security: WiFi King - [link](https://szkolasecurity.pl/wifi/)
        6. Szkoła Security: Bezpieczeństwo API i GraphQL - [link](https://szkolasecurity.pl/bezpieczenstwo-api-i-graphql/)
        7. Docker Maestro - [link](https://dockermaestro.pl/)
        8. Automation Maestro - [link](https://automationmaestro.pl/)
        9. Helm Maestro - [link](https://helmmaestro.pl/)
        10. Kubernetes Maestro - [link](https://kubernetesmaestro.pl/)
        11. Auditor Wiodący Systemu Zarządzania Bezpieczeństwem Informacji wg ISO/IEC 27001:2022 - [link](https://www.bsigroup.com/pl-PL/ISO-IEC-27001-Bezpieczenstwo-Informacji/Szkolenia-dla-ISO-IEC-27001/Auditor-wiodacy/)
        12. Auditor Wewnętrzny Systemu Zarządzania Bezpieczeństwem Informacji wg ISO/IEC 27001:2022 - [link](https://www.bsigroup.com/pl-PL/ISO-IEC-27001-Bezpieczenstwo-Informacji/Szkolenia-dla-ISO-IEC-27001/Auditor-Wewnetrzny/)
        13. AI Devs - [link](https://www.aidevs.pl/)
        14. Cyfrowe Ataki: Jak nie dać się oszukać - [link](https://sklep.szurek.tv/cyfrowe-ataki)
        15. Bezpieczny Programista: Odkryj tajemnice bezpieczeństwa aplikacji webowych! - [link](https://sklep.szurek.tv/bezpieczny-programista)"
        assert_eq!(2 + 2, 4);"#;
        let expected_links = [
        ("Szkoła Security: Security Starter", "https://l.szkolasecurity.pl/starter/"),
        ("Szkoła Security2: Security Starter2", "https://l.szkolasecurity.pl/starter2/"),
        ("Szkoła Security: Testowanie Bezpieczeństwa Web Aplikacji Masters", "https://szkolasecurity.pl/tbwam/"),
        ("Szkoła Security: Security Analyst Krok Pierwszy", "https://szkolasecurity.pl/blue1/"),
        ("Szkoła Security: Python Dla Bezpieczników", "https://szkolasecurity.pl/python/"),
        ("Szkoła Security: WiFi King", "https://szkolasecurity.pl/wifi/"),
        ("Szkoła Security: Bezpieczeństwo API i GraphQL", "https://szkolasecurity.pl/bezpieczenstwo-api-i-graphql/"),
        ("Docker Maestro", "https://dockermaestro.pl/"),
        ("Automation Maestro", "https://automationmaestro.pl/"),
        ("Helm Maestro", "https://helmmaestro.pl/"),
        ("Kubernetes Maestro", "https://kubernetesmaestro.pl/"),
        ("Auditor Wiodący Systemu Zarządzania Bezpieczeństwem Informacji wg ISO/IEC 27001:2022", "https://www.bsigroup.com/pl-PL/ISO-IEC-27001-Bezpieczenstwo-Informacji/Szkolenia-dla-ISO-IEC-27001/Auditor-wiodacy/"),
        ("Auditor Wewnętrzny Systemu Zarządzania Bezpieczeństwem Informacji wg ISO/IEC 27001:2022", "https://www.bsigroup.com/pl-PL/ISO-IEC-27001-Bezpieczenstwo-Informacji/Szkolenia-dla-ISO-IEC-27001/Auditor-Wewnetrzny/"),
        ("AI Devs", "https://www.aidevs.pl/"),
        ("Cyfrowe Ataki: Jak nie dać się oszukać", "https://sklep.szurek.tv/cyfrowe-ataki"),
        ("Bezpieczny Programista: Odkryj tajemnice bezpieczeństwa aplikacji webowych!", "https://sklep.szurek.tv/bezpieczny-programista"),
        ];
        let extracted_links = extract_titles_and_urls(CONTENT);
        assert_eq!(extracted_links, expected_links)
    }

    #[test]
    fn extracts_every_link_bscp() {
        const CONTENT: &str = r#"# Not The Hidden Wiki

        💥 BSCP Notes
        -----
        
        1. BSCP Methodology - [link](https://bscpcheatsheet.gitbook.io/exam)
        2. My experience (and tips) on the Burp Suite Certified Practitioner exam - [link](https://thelicato.medium.com/my-experience-and-tips-on-the-burp-suite-certified-practitioner-exam-1834e31465f8)
        3. Ultimate Burp Suite Exam and PortSwigger Labs Guide - [link](https://github.com/DingyShark/BurpSuiteCertifiedPractitioner)
        4. All of the information I have gathered during the BSCP - [link](https://github.com/jacobocasado/bscp)"#;
        let expected_links = [
        ("BSCP Methodology", "https://bscpcheatsheet.gitbook.io/exam"),
        ("My experience (and tips) on the Burp Suite Certified Practitioner exam", "https://thelicato.medium.com/my-experience-and-tips-on-the-burp-suite-certified-practitioner-exam-1834e31465f8"),
        ("Ultimate Burp Suite Exam and PortSwigger Labs Guide", "https://github.com/DingyShark/BurpSuiteCertifiedPractitioner"),
        ("All of the information I have gathered during the BSCP", "https://github.com/jacobocasado/bscp")];
        let extracted_links = extract_titles_and_urls(CONTENT);
        assert_eq!(extracted_links, expected_links)
    }
    #[test]
    fn extracts_every_link_with_custom_title() {
        const CONTENT: &str = r#"# Not The Hidden Wiki

        💥 BSCP Notes
        -----
        
        1. [exam](https://bscpcheatsheet.gitbook.io/exam) [exam2](https://bscpcheatsheet.gitbook.io/exam2)"#;
        let expected_links = [
            ("exam", "https://bscpcheatsheet.gitbook.io/exam"),
            ("exam2", "https://bscpcheatsheet.gitbook.io/exam2"),
        ];
        let extracted_links = extract_titles_and_urls(CONTENT);
        assert_eq!(extracted_links, expected_links)
    }
}
