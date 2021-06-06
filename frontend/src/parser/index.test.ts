import { parseUrl } from "./index";

describe("parse tweet url", () => {
  test("parse valid url", () => {
    expect(
      parseUrl("https://twitter.com/kenkoooo/status/1368129689965842432")
    ).toBe(1368129689965842432);
    expect(
      parseUrl("https://twitter.com/baechurenes/status/1401444094027517958")
    ).toBe(1401444094027517958);
  });
  test("parse invalid url", () => {
    expect(parseUrl("https://twitter.com/kenkoooo")).toBeUndefined();
    expect(
      parseUrl("https://twitter.com/kenkoooo/status/aaaaa")
    ).toBeUndefined();
  });
});
