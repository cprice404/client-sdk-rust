import * as fs from 'node:fs';
import {cacheWeatherData} from './weather-loader';

async function main() {
  const cwd = process.cwd();
  console.log(`cwd: ${cwd}`);
  const inputFilePath = '../../../../../scratch/weather_16.json';
  const readStream = fs.createReadStream(inputFilePath);
  console.log(`Reading weather data from ${inputFilePath}`);
  await cacheWeatherData(readStream);
  console.log('Back in main, finished caching weather data');
}

main().catch(e => {
  console.error(e);
  throw e;
});
