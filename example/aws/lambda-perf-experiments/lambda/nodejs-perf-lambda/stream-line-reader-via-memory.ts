import {StreamLineReader} from './stream-line-reader';
import * as readline from 'node:readline';
import * as events from 'node:events';

export class StreamLineReaderViaMemory implements StreamLineReader {
  static async open(readStream: NodeJS.ReadableStream): Promise<StreamLineReader> {
    const lines: string[] = [];

    const rl = readline.createInterface({
      input: readStream,
      crlfDelay: Infinity,
    });

    rl.on('line', line => {
      // console.log(`StreamLineReaderViaMemory: read line: ${line}`);
      lines.push(line);
    });

    await events.once(rl, 'close');

    console.log('StreamLineReaderViaMemory: read all lines');

    return new StreamLineReaderViaMemory(lines);
  }

  private readonly lines: string[];

  private constructor(lines: string[]) {
    this.lines = lines;
  }

  readLine(): Promise<string | undefined> {
    return Promise.resolve(this.lines.pop());
  }

  close(): Promise<void> {
    return Promise.resolve();
  }
}
