// Common base types

export const PROGRAM_ENUMS = [
  "Lido",
  "Marinade",
  "ReservePool",
  "SanctumSpl",
  "SanctumSplMulti",
  "Spl",
  "SPool",
] as const;

export type ProgramEnum = (typeof PROGRAM_ENUMS)[number];

export type BasePool = {
  program: ProgramEnum;
};

export type TypedBasePool<P extends ProgramEnum> = {
  program: P;
};

type BaseLST = {
  name: string;
  symbol: string;
  mint: string;
  decimals: number;
};

// Input types - same as actual fields but with snake_case names to match those in toml

type InputProgramToPool = {
  Lido: {};
  Marinade: {};
  ReservePool: {};
  SanctumSpl: InputSplPoolAccounts;
  SanctumSplMulti: InputSplPoolAccounts;
  Spl: InputSplPoolAccounts;
  SPool: InputSPoolAccounts;
};

type InputSplPoolAccounts = {
  pool: string;
  validator_list: string;
  vote_account?: string;
};

type InputSPoolAccounts = {
  program_id: string;
};

type InputPool<P extends BasePool> = P & InputProgramToPool[P["program"]];

type InputLST<P extends BasePool> = BaseLST & {
  pool: InputPool<P>;
  token_program: string;
  logo_uri: string;
};

export type ParseResult = {
  sanctum_lst_list: InputLST<BasePool>[];
};

// Non-input types

export type ProgramToPool = {
  Lido: {};
  Marinade: {};
  ReservePool: {};
  SanctumSpl: SplPoolAccounts;
  SanctumSplMulti: SplPoolAccounts;
  Spl: SplPoolAccounts;
  SPool: SPoolAccounts;
};

export type SplPoolAccounts = {
  pool: string;
  validatorList: string;
  voteAccount?: string;
};

export type SPoolAccounts = {
  programId: string;
};

export type Pool<P extends BasePool> = P & ProgramToPool[P["program"]];

/**
 * A LST of a specific pool type
 */
export type TypedLst<P extends BasePool> = BaseLST & {
  pool: Pool<P>;
  tokenProgram: string;
  logoUri: string;
};

export type LST = TypedLst<BasePool>;
