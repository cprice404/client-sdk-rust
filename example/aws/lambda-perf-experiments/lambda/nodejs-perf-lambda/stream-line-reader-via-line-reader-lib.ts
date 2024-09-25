import * as fs from 'node:fs';
import line_reader = require('line-reader');
import {StreamLineReader} from './stream-line-reader';

export class StreamLineReaderViaLineReaderLib implements StreamLineReader {
  private readonly readStream: fs.ReadStream;
  private readonly lineReader: Reader;
  private hasLines = true;

  static async open(readStream: fs.ReadStream): Promise<StreamLineReader> {
    const lineReader = await new Promise<Reader>((resolve, reject) => {
      line_reader.open(readStream, (err, reader) => {
        if (err) {
          reject(err);
        } else {
          resolve(reader);
        }
      });
    });

    return new StreamLineReaderViaLineReaderLib(readStream, lineReader);
  }

  private constructor(readStream: fs.ReadStream, lineReader: Reader) {
    this.readStream = readStream;
    this.lineReader = lineReader;
  }

  readLine(): Promise<string | undefined> {
    return new Promise<string | undefined>((resolve, reject) => {
      if (!this.hasLines) {
        resolve(undefined);
      } else if (!this.lineReader.hasNextLine()) {
        this.hasLines = false;
        resolve(undefined);
      } else {
        this.lineReader.nextLine((err, line) => {
          if (err) {
            // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
            console.log(`nextLine callback got an err: ${err}`);
            reject(err);
          } else {
            // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
            // console.log(`nextLine callback got a line: ${line}`);
            resolve(line);
          }
        });
      }
    });
  }

  close(): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      this.lineReader.close(err => {
        if (err) {
          reject(err);
        }
        this.readStream.close(err => {
          if (err) {
            reject(err);
          } else {
            resolve();
          }
        });
      });
    });
  }
}
