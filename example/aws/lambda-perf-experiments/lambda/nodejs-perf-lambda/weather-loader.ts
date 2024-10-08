import * as fs from 'node:fs';
import {CacheClient, CacheSetResponse, Configurations, IMomentoCache} from '@gomomento/sdk';
// import {StreamLineReaderViaLineReaderLib} from './stream-line-reader-via-line-reader-lib';
import {StreamLineReaderViaMemory} from './stream-line-reader-via-memory';

async function cacheWeatherDataPoint(cache: IMomentoCache, line: string): Promise<void> {
  // console.log(`Parsing JSON for line: ${line}`);
  // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
  const dataPoint = JSON.parse(line);
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const city = dataPoint.city.name as string;
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const minTemp = dataPoint.main.temp_min as number;
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
  const maxTemp = dataPoint.main.temp_max as number;
  // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
  // console.log(`Caching weather data for ${city}: min=${minTemp}, max=${maxTemp}`);
  const setResponse = await cache.set(city, JSON.stringify({minTemp, maxTemp}));
  switch (setResponse.type) {
    case CacheSetResponse.Success:
      break;
    case CacheSetResponse.Error:
      console.log(`Error caching weather data for ${city}: ${setResponse.toString()}`);
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

export async function cacheWeatherData(readStream: NodeJS.ReadableStream): Promise<void> {
  const cacheClient = await CacheClient.create({
    configuration: Configurations.InRegion.Default.latest().withNumConnections(10).withClientTimeoutMillis(20 * 1000),
    defaultTtlSeconds: 60 * 60,
  });

  const cache = cacheClient.cache('cache');

  console.log('Cache client created, opening stream reader');
  // const reader = await StreamLineReaderViaLineReaderLib.open(readStream);
  const reader = await StreamLineReaderViaMemory.open(readStream);

  console.log('Stream reader opened');

  const numWorkers = 1000;

  const getLine = () => reader.readLine();

  console.log(`Starting ${numWorkers} workers`);
  const workerPromises = startWorkers(numWorkers, cache, getLine);

  console.log('Waiting for all workers to complete');
  const workerResults = await Promise.all(workerPromises);
  console.log('All workers completed');

  await reader.close();

  const lineCount = workerResults.reduce((acc, count) => acc + count, 0);

  console.log(`Read ${lineCount} lines`);
}
