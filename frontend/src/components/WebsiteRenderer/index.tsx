import { useState } from 'react';
import styles from './index.module.css';

const WebsiteRenderer = ({ website }: { website: SearchResult }) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const handleOnLoad = () => {
    setIsLoaded(true);
  };
  return (
    <div
      className={styles.Container}
      style={{
        // paddingBottom: `${(200 / 400) * 100}%`,
        paddingBottom: 216,
        width: 400,
      }}
    >
      <img
        alt={website.title}
        className={styles.ImageThumb}
        src={`data:image/jpeg;base64,/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAbADIDAREAAhEBAxEB/9sAQwAUDg8SDw0UEhASFxUUGB4yIR4cHB49LC4kMklATEtHQEZFUFpzYlBVbVZFRmSIZW13e4GCgU5gjZeMfZZzfoF8/9sAQwEVFxceGh47ISE7fFNGU3x8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8fHx8/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/${website.thumbnail}`}
        style={{ opacity: +!isLoaded }}
        width={400}
      />
      <img
        onLoad={handleOnLoad}
        className={styles.Image}
        style={{ opacity: +isLoaded }}
        srcSet={`/nthw_search_engine/resized_screenshots/${website.id}.png 455w, /nthw_search_engine/screenshots/${website.id}.png 1600w`}
        alt={website.title}
        width={400}
      />
    </div>
  );
};

export default WebsiteRenderer;
