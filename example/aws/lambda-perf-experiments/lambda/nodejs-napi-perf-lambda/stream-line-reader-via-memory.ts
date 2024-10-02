import * as fs from 'node:fs';
import {StreamLineReader} from './stream-line-reader';
import * as readline from 'node:readline';
import * as events from 'node:events';

export class StreamLineReaderViaMemory implements StreamLineReader {
  static async open(readStream: fs.ReadStream): Promise<StreamLineReader> {
    const lines: string[] = [];

    const rl = readline.createInterface({
      input: readStream,
      crlfDelay: Infinity,
    });

    rl.on('line', line => {
      lines.push(line);
    });

    await events.once(rl, 'close');

    return new StreamLineReaderViaMemory(lines);
  }

  private readonly lines: string[];

  private constructor(lines: string[]) {
    this.lines = lines;
  }

  readLine(): Promise<string | undefined> {
    return Promise.resolve(this.lines.pop());
  }

  allLines(): Promise<Array<string>> {
    return Promise.resolve(this.lines);
  }

  close(): Promise<void> {
    return Promise.resolve();
  }
}
