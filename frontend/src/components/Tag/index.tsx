import { ReactNode } from 'react';
import styles from './index.module.css';
import LoadingSpinner from '../spinner';

export const Tag = ({
  isSelected,
  loading,
  onClick,
  tag,
  text,
}: {
  isSelected: boolean;
  loading?: boolean;
  onClick: (_tag: number) => void;
  tag: number;
  text: ReactNode;
}) => {
  const handleClick = () => {
    onClick(tag);
  };
  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-static-element-interactions
    <div
      className={styles.Tag}
      onClick={handleClick}
      style={{ background: isSelected ? '#dd532b' : '#000' }}
    >
      {text}
      {loading === false && <LoadingSpinner color="#fff" size={20} />}
    </div>
  );
};
