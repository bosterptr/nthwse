import json
import torch
import torch.nn.functional as F
from transformers import BertTokenizer, BertModel
from typing import List
from tqdm import tqdm

class WebsiteData:
    def __init__(self, id: int, title: str, content: str, url: str, description: str, tags: List[int]):
        self.id = id
        self.title = title
        self.content = content
        self.url = url
        self.description = description
        self.tags = tags

class WebsiteEmbeddings:
    def __init__(self, id: int, embeddings: List[List[float]]):
        self.id = id
        self.embeddings = embeddings

def chunk_text(text: str, chunk_size: int = 100) -> List[str]:
    words = text.split()
    chunks = []
    for i in range(0, len(words), chunk_size):
        chunk = ' '.join(words[i:i + chunk_size])
        chunks.append(chunk)
    return chunks

def extract_chunk_embeddings(chunk: str, model, tokenizer, device):
    inputs = tokenizer(chunk, return_tensors='pt', truncation=True, padding=True)
    inputs.to(device)
    with torch.no_grad():
        outputs = model(**inputs)
        # Mean pooling: Take the mean of the last hidden state (embedding) for each chunk
        embeddings = outputs.last_hidden_state.mean(dim=1).squeeze()
        normalized_embeddings = F.normalize(embeddings, p=2, dim=0).cpu().numpy().tolist()
        assert len(normalized_embeddings) == 768
    return normalized_embeddings

def process_website_data(item: WebsiteData, model, tokenizer, device):
    content = item.content
    chunks = chunk_text(content)
    content_embeddings = [
        extract_chunk_embeddings(item.title, model, tokenizer, device),
        extract_chunk_embeddings(item.description, model, tokenizer, device)
    ]
    for chunk in chunks:
        chunk_embeddings = extract_chunk_embeddings(chunk, model, tokenizer, device)
        content_embeddings.append(chunk_embeddings)
    website_embeddings = WebsiteEmbeddings(id=item.id, embeddings=content_embeddings)
    return website_embeddings

def extract_and_save_embeddings(data: List[WebsiteData], output_file: str, model_name: str = 'bert-base-uncased'):
    tokenizer = BertTokenizer.from_pretrained(model_name)
    model = BertModel.from_pretrained(model_name)
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    model.to(device)
    model.eval()
    embeddings_list = []
    for item in tqdm(data, desc="Extracting features"):
        website_embeddings = process_website_data(item, model, tokenizer, device)
        embeddings_list.append(website_embeddings)
    with open(output_file, 'w', encoding='utf-8') as f_out:
        json.dump([vars(embeddings) for embeddings in embeddings_list], f_out, ensure_ascii=False, indent=0)

def load_json(file_path: str) -> List[WebsiteData]:
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
        website_data_list = []
        for item in data:
            website_data_list.append(WebsiteData(
                id=item['id'],
                title=item['title'],
                content=item['content'],
                url=item['url'],
                description=item['description'],
                tags=item['tags']
            ))
        return website_data_list

if __name__ == '__main__':
    json_file = '../scraper/websites_full.json'
    data_list = load_json(json_file)
    output_json_file = 'website_embeddings.json'
    extract_and_save_embeddings(data_list, output_json_file)
