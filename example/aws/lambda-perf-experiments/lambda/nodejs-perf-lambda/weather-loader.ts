import * as fs from 'node:fs';
import {StreamLineReader} from './stream-reader';
import {CacheClient, CacheSetResponse, IMomentoCache} from '@gomomento/sdk';

async function cacheWeatherDataPoint(cache: IMomentoCache, line: string): Promise<void> {
  console.log(`Parsing JSON for line: ${line}`);
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
  const dataPoint = JSON.parse(line);
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const city = dataPoint.city.name as string;
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const minTemp = dataPoint.main.temp_min as number;
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const maxTemp = dataPoint.main.temp_max as number;
  // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  console.log(`Caching weather data for ${city}: min=${minTemp}, max=${maxTemp}`);
  const setResponse = await cache.set(city, JSON.stringify({minTemp, maxTemp}));
  switch (setResponse.type) {
    case CacheSetResponse.Success:
      break;
    case CacheSetResponse.Error:
      throw new Error(`Error caching weather data for ${city}: ${setResponse.toString()}`);
  }
}

function startWorkers(
  numWorkers: number,
  cache: IMomentoCache,
  getLine: () => Promise<string | undefined>
): Promise<number>[] {
  return Array.from({length: numWorkers}, async (_, i) => {
    let lineCount = 0;
    let line = await getLine();
    while (line !== undefined) {
      if (line !== '') {
        await cacheWeatherDataPoint(cache, line);
        lineCount++;
      }
      line = await getLine();
    }
    console.log(`Worker ${i} finished`);
    return lineCount;
  });
}

export async function cacheWeatherData(readStream: fs.ReadStream): Promise<void> {
  const cacheClient = await CacheClient.create({
    defaultTtlSeconds: 60 * 60,
  });

  const cache = cacheClient.cache('cache');

  const reader = await StreamLineReader.open(readStream);

  const numWorkers = 20;

  const getLine = () => reader.readLine();

  const workerPromises = startWorkers(numWorkers, cache, getLine);

  const workerResults = await Promise.all(workerPromises);
  const lineCount = workerResults.reduce((acc, count) => acc + count, 0);

  console.log(`Read ${lineCount} lines`);
}
