/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable react/button-has-type */
// eslint-disable-next-line import/no-extraneous-dependencies
import { useEffect, useRef, useState } from 'react';
import styles from './App.module.css';
import { Background } from './components/Background';
import { HitItem } from './components/HitItem';
import { Input } from './components/Input';
import { Tag } from './components/Tag';
import { SearchMode } from './constants';
import tagsJson from './generated/tags.json';
import { FuzzySearcher } from './utils/fuzzySearch';
import { ModelSearcher } from './utils/model';
import { useThrottle } from './utils/useThrottle';

const ITEM_COUNT = 16;

// const usePrevious = (value: any, initialValue: any) => {
//   const ref = useRef(initialValue);
//   useEffect(() => {
//     ref.current = value;
//   });
//   return ref.current;
// };

// const useEffectDebugger = (
//   effectHook: any,
//   dependencies: React.DependencyList,
//   dependencyNames = [],
// ) => {
//   const previousDeps = usePrevious(dependencies, []);

//   const changedDeps = dependencies.reduce((accum, dependency, index) => {
//     if (dependency !== previousDeps[index]) {
//       const keyName = dependencyNames[index] || index;
//       return {
//         ...(accum as any),
//         [keyName]: {
//           before: previousDeps[index],
//           after: dependency,
//         },
//       };
//     }

//     return accum;
//   }, {});

//   if (Object.keys(changedDeps as any).length) {
//     console.log('[use-effect-debugger] ', changedDeps);
//   }

//   // eslint-disable-next-line react-hooks/exhaustive-deps
//   useEffect(effectHook, dependencies);
// };

const App = () => {
  const [query, setQuery] = useState('');
  const [touched, setTouched] = useState(false);
  const [selectedTags, setSelectedTags] = useState<number[]>([]);
  const [results, setResults] = useState<SearchResult[]>([]);
  const [mode, setMode] = useState<SearchMode>(SearchMode.fuzzy);
  const [isFuzzySearcherReady, setIsFuzzySearcherReady] = useState<boolean>(false);
  const [isEmbeddingSearcherReady, setEmbeddingSearcherReady] = useState<boolean>(false);
  const isCalculatingRef = useRef<boolean>(false);
  const [isWasmModuleLoading, setIdWasmModuleLoading] = useState<boolean>(true);
  const modelSearcherRef = useRef(new ModelSearcher());
  // const gridRef = useRef<HTMLDivElement>(null);
  const FuzzSearcherRef = useRef<FuzzySearcher | null>(null);
  const throttledQuery = useThrottle(query, 300);
  useEffect(() => {
    // if (gridRef.current)
    //   wrapGrid(gridRef.current, { easing: 'backOut', stagger: 10, duration: 300 });
    const initWasm = async () => {
      let threads = navigator.hardwareConcurrency;
      if (threads > 8) threads = 8;
      const fuzzySearcher = new FuzzySearcher(threads);
      await fuzzySearcher.init();
      setIsFuzzySearcherReady(true);
      FuzzSearcherRef.current = fuzzySearcher;
      setIdWasmModuleLoading(false);
      await modelSearcherRef.current.init();
      setEmbeddingSearcherReady(true);
    };
    initWasm();
  }, []);
  useEffect(() => {
    const search = async () => {
      if (mode === SearchMode.fast) {
        let searchResult: SearchResult[] = [];
        if (FuzzSearcherRef.current) {
          const existingKeys: number[] = [];
          const searchResponseCollections = await FuzzSearcherRef.current.search(
            query,
            selectedTags,
            mode === SearchMode.fast,
          );
          for (const searchResponseCollection of searchResponseCollections) {
            for (const searchResults of searchResponseCollection) {
              if (existingKeys.indexOf(searchResults.id) === -1) {
                searchResult.push(searchResults);
                existingKeys.push(searchResults.id);
              }
            }
          }
        }
        searchResult = searchResult
          .filter((a) => a.score)
          .sort((a, b) => b.score - a.score)
          .slice(0, ITEM_COUNT);
        setResults(searchResult);
      }
    };
    search();
  }, [isWasmModuleLoading, mode, query, selectedTags]);
  useEffect(() => {
    const search = async () => {
      if (isCalculatingRef.current) return;
      switch (mode) {
        case SearchMode.fuzzy: {
          let searchResult: SearchResult[] = [];
          if (FuzzSearcherRef.current) {
            const existingKeys: number[] = [];
            isCalculatingRef.current = true;
            const searchResponseCollections = await FuzzSearcherRef.current.search(
              throttledQuery,
              selectedTags,
              false,
            );
            for (const searchResponseCollection of searchResponseCollections) {
              for (const searchResults of searchResponseCollection) {
                if (existingKeys.indexOf(searchResults.id) === -1) {
                  searchResult.push(searchResults);
                  existingKeys.push(searchResults.id);
                }
              }
            }
          }
          searchResult = searchResult
            .filter((a) => a.score)
            .sort((a, b) => b.score - a.score)
            .slice(0, ITEM_COUNT);
          setResults(searchResult);
          isCalculatingRef.current = false;
          // eslint-disable-next-line no-console
          break;
        }
        case SearchMode.ai: {
          if (
            modelSearcherRef.current.isInitialized &&
            !isWasmModuleLoading &&
            FuzzSearcherRef.current
          ) {
            isCalculatingRef.current = true;
            const searchResult = await modelSearcherRef.current.search(
              throttledQuery,
              selectedTags,
            );
            setResults(searchResult);
            isCalculatingRef.current = false;
          }
          break;
        }
        default: {
          /* empty */
        }
      }
    };
    search();
  }, [isWasmModuleLoading, mode, selectedTags, throttledQuery]);
  // const handleExportDict = async () => {
  //   if (!exportingDict) {
  //     if (
  //       modelSearcherRef.current.isInitialized &&
  //       !isWasmModuleLoading &&
  //       FuzzSearcherRef.current
  //     ) {
  //       console.log('starting');
  //       setIsExportingDict(true);
  //       await modelSearcherRef.current.exportDict();
  //       // const similarities = await modelSearcherRef.current.similarity(throttledQuery);
  //       // const searchResult = await FuzzSearcherRef.current.similaritiesToSearchResults(
  //       //   similarities,
  //       //   selectedTags,
  //       // );
  //       // setResults(searchResult);
  //     }
  //   }
  // };
  const handleTagClick = (tag: number) => {
    if (selectedTags.includes(tag)) {
      const updatedTags = selectedTags.filter((selectedTag) => selectedTag !== tag);
      setSelectedTags([...updatedTags]);
    } else {
      const updatedSelectedTags = [...selectedTags, tag];
      setSelectedTags(updatedSelectedTags);
    }
  };
  const handleInputChange = (data: string) => {
    setQuery(data);
    if (!touched) setTouched(true);
  };
  const hasText = query !== '';
  return (
    <div className={styles.App}>
      <Background />
      {/* <input
        type="checkbox"
        checked={mode === SearchMode.fast}
        onChange={handleClickFastSearch}
      />{' '}
      Fast
      <input type="checkbox" checked={mode === SearchMode.ai} onChange={handleClickAiSearch} /> AI */}

      <h1 className={styles.H1} style={{ opacity: query !== '' || touched ? 0 : 1 }}>
        Not The Hidden Wiki
      </h1>
      <div
        className={styles.InputContainer}
        style={{
          marginTop: hasText || touched ? '2%' : '20%',
        }}
      >
        <Input
          isEmbeddingSearcherReady={isEmbeddingSearcherReady}
          isFuzzySearcherReady={isFuzzySearcherReady}
          mode={mode}
          onChange={handleInputChange}
          setMode={setMode}
          value={query}
        />
      </div>
      <div className={styles.ListContainer} style={{ opacity: query !== '' || touched ? 1 : 0 }}>
        <div className={styles.TagList}>
          {tagsJson.map((name, index) => (
            <Tag
              // eslint-disable-next-line react/no-array-index-key
              key={index}
              text={name}
              tag={index}
              onClick={handleTagClick}
              isSelected={selectedTags.includes(index)}
            />
          ))}
        </div>
        {results.map((website) => (
          <div key={website.id}>
            <HitItem
              website={website}
              selectedTags={selectedTags}
              setSelectedTags={setSelectedTags}
            />
          </div>
        ))}
      </div>
    </div>
  );
};

export default App;
