import * as fs from 'node:fs';
import line_reader = require('line-reader');

export class StreamLineReader {
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

    return new StreamLineReader(readStream, lineReader);
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
            reject(err);
          } else {
            resolve(line);
          }
        });
      }
    });
  }
}
