const dots: [string, number][] = [
  ['80 50', 0.875],
  ['45 -50.355 121.569', 0.75],
  ['90 -15 65', 0.625],
  ['135 -.355 41.569', 0.5],
  ['180 10 25', 0.375],
  ['-135 20.355 8.431', 0.25],
  ['-90 35 -15', 0.125],
  ['-45 70.355 -71.569', 0],
];

const LoadingSpinner = ({
  className,
  size = 32,
  color = 'rgb(0 87 242)',
}: {
  className?: string;
  size?: string | number;
  color?: string;
}) => (
  <svg
    width={size}
    height={size}
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 100 100"
    preserveAspectRatio="xMidYMid"
    className={className}
  >
    {dots.map(([string, value], index) => (
      <g key={string}>
        <g transform={`${index ? 'rotate' : 'translate'}(${string})`}>
          <circle r={8} fill={color} fillOpacity={value + 0.125}>
            <animateTransform
              attributeName="transform"
              type="scale"
              begin={`-${value}s`}
              values="1 1;1 1"
              keyTimes="0;1"
              dur="1s"
              repeatCount="indefinite"
            />
            <animate
              attributeName="fill-opacity"
              keyTimes="0;1"
              dur="1s"
              repeatCount="indefinite"
              values="1;0"
              begin={`-${value}s`}
            />
          </circle>
        </g>
      </g>
    ))}
  </svg>
);

export default LoadingSpinner;
