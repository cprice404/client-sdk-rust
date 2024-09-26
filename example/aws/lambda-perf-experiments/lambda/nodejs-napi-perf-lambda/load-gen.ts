import {CacheClient, CacheGetResponse, CacheSetResponse, Configurations, CredentialProvider} from '@gomomento/sdk';
import * as hdr from 'hdr-histogram-js';

const CACHE_NAME = 'cache';

export async function runLoadGen() {
  const startTime = Date.now();
  console.log('Hello from loadgen!');
  const cacheClient = await CacheClient.create({
    configuration: Configurations.Lambda.latest().withClientTimeoutMillis(10_000),
    credentialProvider: CredentialProvider.fromEnvVar('MOMENTO_API_KEY'),
    defaultTtlSeconds: 60,
  });

  const setHistogram = hdr.build();
  const getHistogram = hdr.build();

  const numWorkers = 200;
  const runTimeSeconds = 30;
  const workerPromises = range(numWorkers).map(i =>
    runWorker(i, cacheClient, setHistogram, getHistogram, runTimeSeconds)
  );
  await Promise.all(workerPromises);

  const setP50 = setHistogram.getValueAtPercentile(50);
  console.log(`Set p50: ${setP50}`);
  const getP50 = getHistogram.getValueAtPercentile(50);
  console.log(`Get p50: ${getP50}`);

  const totalRequestCount = setHistogram.totalCount + getHistogram.totalCount;
  console.log(`Total requests: ${totalRequestCount}`);
  const tps = totalRequestCount / ((Date.now() - startTime) / 1000);
  console.log(`TPS: ${tps}`);

  printHistogramSummary(setHistogram, 'Set');
  printHistogramSummary(getHistogram, 'Get');
}

async function runWorker(
  workerIndex: number,
  cacheClient: CacheClient,
  setHistogram: hdr.Histogram,
  getHistogram: hdr.Histogram,
  runTimeSeconds: number
) {
  const key = `key-${workerIndex}`;
  const value = `value-${workerIndex}`;

  const startTime = Date.now();
  while (Date.now() - startTime < runTimeSeconds * 1000) {
    const setStart = Date.now();
    const setResponse = await cacheClient.set(CACHE_NAME, key, value);
    switch (setResponse.type) {
      case CacheSetResponse.Success:
        break;
      case CacheSetResponse.Error:
        throw new Error(`Error setting cache value: ${setResponse.toString()}`);
    }
    setHistogram.recordValue(Date.now() - setStart);

    const getStart = Date.now();
    const getResponse = await cacheClient.get(CACHE_NAME, key);
    switch (getResponse.type) {
      case CacheGetResponse.Error:
        throw new Error(`Error setting cache value: ${getResponse.toString()}`);
      default:
        break;
    }
    getHistogram.recordValue(Date.now() - getStart);
  }
}

function printHistogramSummary(histogram: hdr.Histogram, name: string) {
  console.log(`cumulative ${name} latencies:`);
  console.log(`  count: ${histogram.totalCount}`);
  console.log(`    min: ${histogram.minNonZeroValue}`);
  console.log(`    p50: ${histogram.getValueAtPercentile(50)}`);
  console.log(`    p90: ${histogram.getValueAtPercentile(90)}`);
  console.log(`    p99: ${histogram.getValueAtPercentile(99)}`);
  console.log(`  p99.9: ${histogram.getValueAtPercentile(99.9)}`);
  console.log(`    max: ${histogram.maxValue}`);
}

function range(n: number) {
  return Array.from({length: n}, (_, i) => i);
}
