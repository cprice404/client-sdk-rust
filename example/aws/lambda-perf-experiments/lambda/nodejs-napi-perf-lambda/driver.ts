import {runLoadGen} from './load-gen';

async function main() {
  await runLoadGen();
}

main().catch(e => {
  console.error(e);
  throw e;
});
