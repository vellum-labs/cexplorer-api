export interface ResponseCore<T> {
  code: number;
  data: T;
  tokens: number;
  ex: number;
  debug: boolean;
}
