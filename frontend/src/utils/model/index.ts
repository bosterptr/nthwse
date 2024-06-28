import { Remote, wrap } from 'comlink';
import ComlinkWorker, { EmbeddingExtractionWorker } from './embedding_extractor.worker';
// import websiteData from '../../generated/websites_full.json';

const saveFile = async (data: string) => {
  const blob = new Blob([data], { type: 'application/json' });
  const link = document.createElement('a');
  link.download = 'embeddings.json';
  link.href = window.URL.createObjectURL(blob);
  document.body.appendChild(link);
  link.click();
  if (link.parentNode) link.parentNode.removeChild(link);
};
function sliceIntoChunks(arr: { id: string; content: string }[], chunkSize: number) {
  const res = [];
  for (let i = 0; i < arr.length; i += chunkSize) {
    const chunk = arr.slice(i, i + chunkSize);
    res.push(chunk);
  }
  return res;
}

export class ModelSearcher {
  private worker: Remote<EmbeddingExtractionWorker> | null;

  public isInitialized: boolean;

  constructor() {
    this.worker = null;
    this.isInitialized = false;
  }

  public async init() {
    const worker: Worker = new ComlinkWorker();
    const WebworkerProcessorApi = wrap<typeof EmbeddingExtractionWorker>(worker);
    this.worker = await new WebworkerProcessorApi();
    await this.worker.init();
    this.isInitialized = true;
  }

  public async search(text: string, filterByTags: number[]): Promise<SearchResult[]> {
    if (!this.worker) throw new Error('you need to initialize ModelSearcher first');
    return this.worker.search(text, filterByTags);
  }

  // public async exportDict() {
  //   sliceIntoChunks(websiteData as unknown as { id: string; content: string }[], 100).forEach(
  //     async (chunk) => {
  //       if (!this.worker) throw new Error('you need to initialize ModelSearcher first');
  //       const result = await this.worker.exportDict(chunk);
  //       saveFile(result);
  //     },
  //   );
  // }
}

// public async export() {

//   embeddingsAsArray = Object.fromEntries(
//       Object.entries(embeddingsDict).map(([key, values]) => [key, Object.values(values).map(roundDecimals)])
//   );

//   const meanEmbedding = calculateAverageEmbedding(embeddingsAsArray)
//   // adding mean embedding so all indexed docs on HF could be ingested in a "proper" vector DB!
//   exportDict = {
//       "meta": message.data.meta, "text": message.data.text,
//       "index": embeddingsAsArray,
//       "mean_embedding": meanEmbedding
//   }

//   exportDict.meta.chunks = Object.keys(embeddingsAsArray).length;

//   console.log("Document average embedding", meanEmbedding);
//   console.log("Metadata", exportDict.meta);

//   gzippedData = pako__WEBPACK_IMPORTED_MODULE_1__["default"].gzip(JSON.stringify(exportDict), { to: 'string' });

//   const tempFilename = `${message.data.meta.textTitle.replace(/\s/g, '_')}_${minimalRandomEightCharHash()}.json.gz`
//   // Send the gzipped data as a response
// }
