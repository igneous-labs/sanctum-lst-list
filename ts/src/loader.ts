import { readFile } from "fs/promises";
import toml from "toml";
import { objectToCamel } from "ts-case-convert";
import type { LST, ParseResult } from "./types";

export class SanctumLstList {
  constructor() {}

  static async loadFromFile(path: string): Promise<LST[]> {
    const fileContents = await readFile(path, "utf8");
    return this.loadFromStr(fileContents);
  }

  static loadFromStr(str: string): LST[] {
    try {
      const parsedData = toml.parse(str) as ParseResult;
      const camelCaseData = objectToCamel(parsedData) as unknown as {
        sanctumLstList: LST[];
      };
      return camelCaseData.sanctumLstList;
    } catch (err) {
      console.error(`Error parsing TOML: ${err}`);
      throw err;
    }
  }

  static load(): Promise<LST[]> {
    return this.loadFromFile("../sanctum-lst-list.toml");
  }
}

export default SanctumLstList;
