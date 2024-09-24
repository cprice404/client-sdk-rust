import * as fs from 'node:fs';
import {StreamLineReader} from './stream-reader';

function startWorkers(numWorkers: number, getLine: () => Promise<string | undefined>): Promise<number>[] {
  return Array.from({length: numWorkers}, async () => {
    let lineCount = 0;
    let line = await getLine();
    while (line !== undefined) {
      lineCount++;
      line = await getLine();
    }
    return lineCount;
  });
}

export async function cacheWeatherData(readStream: fs.ReadStream): Promise<void> {
  const reader = await StreamLineReader.open(readStream);

  const numWorkers = 20;

  const getLine = () => reader.readLine();

  const workerPromises = startWorkers(numWorkers, getLine);

  const workerResults = await Promise.all(workerPromises);
  const lineCount = workerResults.reduce((acc, count) => acc + count, 0);

  console.log(`Read ${lineCount} lines`);
}
