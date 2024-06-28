/* eslint-disable camelcase */
import { expose } from 'comlink';
import initSearchModule, {
  init,
  search,
  substring_search,
} from 'fuzzy_search';

export default {} as typeof Worker & { new (): Worker };

export class WebWorkerFuzzSearcher {
  private loaded = false;

  // eslint-disable-next-line class-methods-use-this
  public async init(workerCount: number, workerIdx: number) {
    await initSearchModule();
    init(workerCount, workerIdx);
    this.loaded = true;
  }

  public async search(query: string, filterByTags: number[]): Promise<SearchResult[]> {
    if (!this.loaded) throw new Error('You need to initialize the worker');
    return search(query, Int32Array.from(filterByTags));
  }

  public async substringSearch(query: string, filterByTags: number[]): Promise<SearchResult[]> {
    if (!this.loaded) throw new Error('You need to initialize the worker');
    return substring_search(query, Int32Array.from(filterByTags));
  }
}

expose(WebWorkerFuzzSearcher);
