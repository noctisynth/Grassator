import { invoke } from '@tauri-apps/api/core';
import { Config } from '../types';

export async function loadConfig(): Promise<Config> {
  return invoke('load_config');
}
