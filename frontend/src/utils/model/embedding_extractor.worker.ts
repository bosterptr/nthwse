/* eslint-disable no-restricted-syntax */
/* eslint-disable camelcase */
import {
  AutoTokenizer,
  // cos_sim,
  env,
  FeatureExtractionPipeline,
  pipeline,
} from '@xenova/transformers';
import { expose } from 'comlink';
import initSearchModule, { search } from 'embeddings_search';
// import websiteEmbeddings from '../../generated/embeddings.json';
// import websiteData from '../../generated/websites_full.json';

// function splitTextIntoWordChunks(text: string, chunkSize: number) {
//   // Split the text into an array of words
//   const words = text.split(/\s+/);

//   // Initialize an array to hold the chunks
//   const chunks = [];

//   // Loop through the words array and create chunks
//   for (let i = 0; i < words.length; i += chunkSize) {
//     // Create a chunk by slicing the words array
//     const chunk = words.slice(i, i + chunkSize);
//     // Join the chunk back into a string and add it to the chunks array
//     chunks.push(chunk.join(' '));
//   }

//   return chunks;
// }

// function splitTextIntoCharChunks(inputString: string, chunkSize = 1000) {
//   const chunks = [];
//   for (let i = 0; i < inputString.length; i += chunkSize) {
//     chunks.push(inputString.slice(i, i + chunkSize));
//   }
//   return chunks;
// }

const MODEL_NAME = 'Xenova/bert-base-uncased';
export default {} as typeof Worker & { new (): Worker };

export class EmbeddingExtractionWorker {
  private tokenizer: Awaited<ReturnType<typeof AutoTokenizer.from_pretrained>> | null;

  private embedder: FeatureExtractionPipeline | null;

  public isInitialized: boolean;

  constructor() {
    this.tokenizer = null;
    this.embedder = null;
    this.isInitialized = false;
    this.isInitialized = false;
    env.allowLocalModels = false;
  }

  public async init() {
    this.tokenizer = await AutoTokenizer.from_pretrained(MODEL_NAME);
    this.embedder = await pipeline('feature-extraction', MODEL_NAME, {
      revision: 'default',
    });
    await initSearchModule();
    // init();
    this.isInitialized = true;
  }

  public async search(text: string, filterByTags: number[]): Promise<SearchResult[]> {
    if (!this.embedder) throw new Error('you need to initialize EmbeddingExtractionWorker first');
    const queryEmbeddings = await this.embedder(text, { pooling: 'mean', normalize: true });
    const results = await search(
      Float32Array.from(queryEmbeddings.data as Float32Array),
      Int32Array.from(filterByTags),
    );
    return results;
  }

  // public async exportDict() {
  //   const { embedder } = this;
  //   if (!embedder) throw new Error('you need to initialize EmbeddingExtractionWorker first');
  //   const promises = websiteData.map((website) => {
  //     console.log(website.id);
  //     return Promise.all(
  //       splitTextIntoWordChunks(website.content, SPLIT_BY_X_WORDS).map((chunk) =>
  //         embedder(chunk, { pooling: 'mean', normalize: true }),
  //       ),
  //     );
  //   });
  //   const response = await Promise.all(promises);
  //   const data = JSON.stringify(
  //     websiteData.map((website, index) => ({
  //       id: website.id,
  //       // eslint-disable-next-line no-restricted-syntax
  //       embeddings: response[index].map((tensor) => Array.from(tensor.data)),
  //     })),
  //   );
  //   console.log(data);
  //   return data;
  // }
}
expose(EmbeddingExtractionWorker);
