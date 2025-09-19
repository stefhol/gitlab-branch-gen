import { EnvConfig } from './types.js';

export function loadEnvConfig(): EnvConfig {
  return {
    GITLAB_URL: process.env['GITLAB_URL'],
    GITLAB_TOKEN: process.env['GITLAB_TOKEN'],
    GITLAB_REPOS: process.env['GITLAB_REPOS'],
  };
}

export function validateEnvConfig(env: EnvConfig): {
  isValid: boolean;
  missing: string[];
} {
  const missing: string[] = [];

  if (!env.GITLAB_URL) missing.push('GITLAB_URL');
  if (!env.GITLAB_TOKEN) missing.push('GITLAB_TOKEN');
  if (!env.GITLAB_REPOS) missing.push('GITLAB_REPOS');

  return {
    isValid: missing.length === 0,
    missing,
  };
}
