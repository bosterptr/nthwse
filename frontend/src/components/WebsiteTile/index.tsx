import styles from './index.module.css';

export const WebsiteTile = ({ website }: { website: SearchResult }) => {
  const handleClick = () => {
    window.location.href = website.url;
  };
  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-static-element-interactions
    <div className={styles.Root} onClick={handleClick}>
      <div className={styles.LogoContainer}>
        <img
          className={styles.LogoRelative}
          srcSet={`/nthw_search_engine/resized_screenshots/${website.id}.png 455w, /nthw_search_engine/screenshots/${website.id}.png 1600w`}
          alt={website.title}
        />
      </div>
      <div className={styles.BottomBar}>
        <h6 className={styles.H2}>{website.title}</h6>
      </div>
    </div>
  );
};
