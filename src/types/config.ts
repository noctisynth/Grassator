export interface Config {
  preset_file?: string;
  num_threads?: number;
}

export interface ConfigResult {
  inner: Config;
  file_path: string;
}
