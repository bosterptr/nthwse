import { Dispatch } from 'react';
import styles from './index.module.css';
import { SearchMode } from '../../constants';
import { Tag } from '../Tag';

export const Input = ({
  isEmbeddingSearcherReady,
  isFuzzySearcherReady,
  mode,
  onChange,
  setMode,
  value,
}: {
  isEmbeddingSearcherReady: boolean;
  isFuzzySearcherReady: boolean;
  mode: SearchMode;
  onChange: (changedValue: string) => void;
  setMode: Dispatch<React.SetStateAction<SearchMode>>;
  value: string;
}) => {
  const handleChange: React.ChangeEventHandler<HTMLInputElement> = async (event) => {
    onChange(event.target.value);
  };
  return (
    <div className={styles.InputContainer}>
      <input
        className={styles.Input}
        value={value}
        onChange={handleChange}
        placeholder="Look for what Demetrius of Phalerum would not be ashamed of"
      />
      <Tag
        text="Fast"
        isSelected={mode === SearchMode.fast}
        tag={SearchMode.fast}
        onClick={setMode}
        loading={isFuzzySearcherReady}
      />
      <Tag
        text="Fuzzy"
        isSelected={mode === SearchMode.fuzzy}
        tag={SearchMode.fuzzy}
        onClick={setMode}
        loading={isFuzzySearcherReady}
      />
      <Tag
        text="AI"
        isSelected={mode === SearchMode.ai}
        tag={SearchMode.ai}
        onClick={setMode}
        loading={isEmbeddingSearcherReady}
      />
    </div>
  );
};
