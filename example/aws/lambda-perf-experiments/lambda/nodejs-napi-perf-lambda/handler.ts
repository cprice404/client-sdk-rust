import {Readable} from 'stream';
import * as zlib from 'node:zlib';
import {cacheWeatherData} from './weather-loader';

export const handler = async () => {
  try {
    const response = await fetch('https://napi-rs-demo.s3.us-west-2.amazonaws.com/weather_16.json.gz');

    // Convert the web ReadableStream to a Node.js Readable stream
    const reader = response.body!.getReader();

    const stream = new Readable({
      async read() {
        const {done, value} = await reader.read();
        if (done) {
          this.push(null); // No more data, end the stream
        } else {
          this.push(value); // Push the chunk into the Node.js stream
        }
      },
    });

    const gunzipStream = stream.pipe(zlib.createGunzip());

    await cacheWeatherData(gunzipStream);
    console.log('Back in main, finished caching weather data');

    return {
      statusCode: 200,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      },
      body: '{}',
    };
  } catch (err) {
    console.log(err);
    return {
      statusCode: 500,
      body: JSON.stringify({
        message: 'An error occurred!',
      }),
    };
  }
};
