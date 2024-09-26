export interface StreamLineReader {
  readLine(): Promise<string | undefined>;
  close(): Promise<void>;
}
