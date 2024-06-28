import { Remote, wrap } from 'comlink';
import ComlinkWorker, { WebWorkerFuzzSearcher } from './index.worker';

export class FuzzySearcher {
  private workerCount: number;

  private workers: Remote<WebWorkerFuzzSearcher>[];

  constructor(workerCount: number) {
    this.workerCount = workerCount;
    this.workers = [];
  }

  public async init() {
    const workerPromises: Promise<Remote<WebWorkerFuzzSearcher>>[] = [];
    for (let workerIdx = 0; workerIdx < this.workerCount; workerIdx += 1) {
      const webworkerSearcherWorker: Worker = new ComlinkWorker();
      const WebworkerProcessorApi = wrap<typeof WebWorkerFuzzSearcher>(webworkerSearcherWorker);
      workerPromises.push(new WebworkerProcessorApi());
    }
    this.workers = await Promise.all(workerPromises);
    const initPromises = [];
    for (let workerIdx = 0; workerIdx < this.workerCount; workerIdx += 1) {
      initPromises.push(this.workers[workerIdx].init(this.workerCount, workerIdx));
    }
    await Promise.all(initPromises);
    return this;
  }

  public async search(
    query: string,
    filterByTags: number[],
    fast: boolean,
  ): Promise<SearchResult[][]> {
    const promises = [];
    for (let workerIdx = 0; workerIdx < this.workerCount; workerIdx += 1) {
      if (fast) promises.push(this.workers[workerIdx].substringSearch(query, filterByTags));
      else promises.push(this.workers[workerIdx].search(query, filterByTags));
    }
    return Promise.all(promises);
  }

  // public async similaritiesToSearchResults(
  //   similarities: number[],
  //   filterByTags: number[],
  // ): Promise<SearchResult[]> {
  //   return this.workers[0].similaritesToSearchResults(similarities, filterByTags);
  // }
}
