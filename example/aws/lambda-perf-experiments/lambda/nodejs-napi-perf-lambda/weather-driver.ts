import * as fs from 'node:fs';
import {cacheWeatherData} from './weather-loader';
// import {WeatherItemCacher} from 'weather_cacher';

// eslint-disable-next-line require-await,@typescript-eslint/require-await
async function main() {
  const cwd = process.cwd();
  console.log(`cwd: ${cwd}`);
  const inputFilePath = '../../../../../scratch/weather_16.json';
  const readStream = fs.createReadStream(inputFilePath);
  console.log(`Reading weather data from ${inputFilePath}`);
  await cacheWeatherData(readStream);
  console.log('Back in main, finished caching weather data');
  // const cacher = WeatherItemCacher.create();
  // cacher.close();
  // console.log(`Called NAPI lib: ${cacher}`);
}

main().catch(e => {
  console.error(e);
  throw e;
});
