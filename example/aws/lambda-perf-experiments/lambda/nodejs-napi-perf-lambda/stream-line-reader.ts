export interface StreamLineReader {
  readLine(): Promise<string | undefined>;
  close(): Promise<void>;

  allLines(): Promise<Array<string>>;
}
