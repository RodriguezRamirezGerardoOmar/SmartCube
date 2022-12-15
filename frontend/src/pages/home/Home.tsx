import { useState } from 'react';
import { useApi } from '@gear-js/react-hooks';
import { ADDRESS } from 'consts';
import { dbMetaWasm } from 'assets/wasm';
import {
  Message,
} from 'types';


// Get messages on channel page
async function useMessages() {
  const { api } = useApi();
  const buffer = await fetch(dbMetaWasm)
    .then((res) => res.arrayBuffer())
    .then((arrayBuffer) => Buffer.from(arrayBuffer))
  const res = await api.programState.read(ADDRESS.DATABASE_CONTRACT, buffer).then((state) => state.toHuman());

  return { res };

}

function Home() {
  console.log(useMessages());
  return <div >Hola</div>;
}

export { Home };
