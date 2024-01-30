type BasePool = {
  program: string;
  pool?: string;
};

type InputPool = BasePool & {
  validator_list?: string;
  vote_account?: string;
};

type Pool = BasePool & {
  validatorList?: string;
  voteAccount?: string;
};

type BaseLST = {
  name: string;
  symbol: string;
  mint: string;
  decimals: number;
};

type InputLST = BaseLST & {
  token_program: string;
  logo_uri: string;
  pool: InputPool;
};

export type LST = BaseLST & {
  tokenProgram: string;
  logoUri: string;
  pool: Pool;
};

export type ParseResult = {
  sanctum_lst_list: InputLST[];
};
