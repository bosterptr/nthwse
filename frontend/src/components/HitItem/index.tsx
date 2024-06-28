/* eslint-disable jsx-a11y/no-static-element-interactions */
/* eslint-disable jsx-a11y/click-events-have-key-events */
import { Dispatch, SetStateAction } from 'react';
import styles from './index.module.css';
import tagsJson from '../../generated/tags.json';
import { Tag } from '../Tag';
import WebsiteRenderer from '../WebsiteRenderer';

export const HitItem = ({
  website,
  selectedTags,
  setSelectedTags,
}: {
  website: SearchResult;
  selectedTags: number[];
  setSelectedTags: Dispatch<SetStateAction<number[]>>;
}) => {
  const handleClick = () => {
    window.location.href = website.url;
  };
  const handleTagClick = (tag: number) => {
    if (selectedTags.includes(tag)) {
      const updatedTags = selectedTags.filter((selectedTag) => selectedTag !== tag);
      setSelectedTags([...updatedTags]);
    } else {
      const updatedSelectedTags = [...selectedTags, tag];
      setSelectedTags(updatedSelectedTags);
    }
  };
  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-static-element-interactions
    <div className={styles.Card}>
      {/* <div>
        <img
          srcSet={`/nthw_search_engine/resized_screenshots/${website.id}.png 455w, /nthw_search_engine/screenshots/${website.id}.png 1600w`}
          alt={website.title}
        />
      </div> */}
      <div className={styles.Clickable} onClick={handleClick}>
        <p className={styles.Title}>{website.title}</p>
        <p className={styles.Description}>{website.description}</p>
        <p className={styles.Description}>{website.url}</p>
        <WebsiteRenderer website={website} />
      </div>
      {website.content}
      <div className={styles.TagsListContainer}>
        {website.tags.map((tag) => (
          <Tag
            key={tag}
            text={tagsJson[tag]}
            tag={tag}
            onClick={handleTagClick}
            isSelected={selectedTags.includes(tag)}
          />
        ))}
      </div>
    </div>
  );
};
