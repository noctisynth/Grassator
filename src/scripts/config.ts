import { invoke } from '@tauri-apps/api/core';
import { Config, ConfigResult, ErrorPayload } from '../types';
import { useToast } from 'primevue/usetoast';

export let config: Config;

export async function loadConfig(): Promise<ConfigResult> {
  const toast = useToast();
  const configResult: ConfigResult = await invoke('load_config');
  config = new Proxy(configResult.inner, {
    set: (target, key, value) => {
      const result = Reflect.set(target, key, value);
      invoke('set_config', {
        newConfig: target,
      }).catch((e: ErrorPayload) => {
        toast.add({ severity: 'error', summary: 'Error', detail: e });
      });
      return result;
    },
  });
  return {
    file_path: configResult.file_path,
    inner: config,
  };
}
