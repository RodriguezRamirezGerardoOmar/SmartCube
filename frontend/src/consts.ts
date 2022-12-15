import { Hex } from '@gear-js/api';

const ADDRESS = {
  NODE: process.env.REACT_APP_NODE_ADDRESS as string,
  DATABASE_CONTRACT: process.env.REACT_APP_DATABASE_CONTRACT_ADDRESS as Hex,
};

const META_STORAGE_ADDRESS = process.env.REACT_APP_META_STORAGE_API as string;

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export { ADDRESS, LOCAL_STORAGE, META_STORAGE_ADDRESS };
