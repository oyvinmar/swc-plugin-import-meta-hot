import { mapProps } from './props';

declare interface ImportMeta {
  readonly hot: Record<string, any>;
}


describe("typescript", () => {
  it("should make import.meta.env use process.env and expose the object", () => {
    console.log(mapProps);

    expect(typeof import.meta.hot).toBe("undefined");
  });
});
