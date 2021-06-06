export const parseUrl = (url: string) => {
  const matches = url.match(/status\/[0-9]+/);
  if (!matches || matches.length === 0) {
    return undefined;
  }

  try {
    return BigInt(matches[0].slice(7));
  } catch (e) {
    console.error(e);
    return undefined;
  }
};
