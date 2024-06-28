/* eslint-disable no-restricted-syntax */
/* eslint-disable camelcase */
import {
  AutoTokenizer,
  env,
  FeatureExtractionPipeline,
  pipeline,
} from '@xenova/transformers';
import { expose } from 'comlink';
import initSearchModule, { search } from 'embeddings_search';

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
}
expose(EmbeddingExtractionWorker);
