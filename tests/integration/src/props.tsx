// Simple JSX component to test that the plugin works with TSX files
type Props = { name: string };

export const mapProps = <T extends Props>(props: T) => {
  return <div>{props.name}</div>;
};
