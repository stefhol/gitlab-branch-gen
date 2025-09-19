import React, { useState, useEffect } from 'react';
import { Box, Text } from 'ink';
import TextInput from 'ink-text-input';
import { Config, EnvConfig } from '../types.js';

interface Props {
  onConfigComplete: (config: Config) => void;
  envConfig: EnvConfig;
  missingEnvVars: string[];
}

export const ConfigInput: React.FC<Props> = ({
  onConfigComplete,
  envConfig,
  missingEnvVars
}) => {
  const [step, setStep] = useState<
    'url' | 'token' | 'repos' | 'branch' | 'complete'
  >('url');
  const [config, setConfig] = useState<Partial<Config>>({
    gitlabUrl: envConfig.GITLAB_URL || '',
    token: envConfig.GITLAB_TOKEN || '',
    repositories: envConfig.GITLAB_REPOS ?
      envConfig.GITLAB_REPOS.split(',').map(r => r.trim()) : [],
  });
  const [currentInput, setCurrentInput] = useState<string>('');

  // Skip steps that are already filled from env vars
  useEffect(() => {
    if (envConfig.GITLAB_URL && step === 'url') {
      setStep('token');
    }
    if (envConfig.GITLAB_TOKEN && step === 'token') {
      setStep('repos');
    }
    if (envConfig.GITLAB_REPOS && step === 'repos') {
      setStep('branch');
    }
  }, [step, envConfig]);

  const handleSubmit = () => {
    switch (step) {
      case 'url':
        setConfig({ ...config, gitlabUrl: currentInput.replace(/\/$/, '') });
        setCurrentInput('');
        setStep('token');
        break;
      case 'token':
        setConfig({ ...config, token: currentInput });
        setCurrentInput('');
        setStep('repos');
        break;
      case 'repos':
        const repositories = currentInput
          .split(',')
          .map((repo) => repo.trim())
          .filter(Boolean);
        setConfig({ ...config, repositories });
        setCurrentInput('');
        setStep('branch');
        break;
      case 'branch':
        const finalConfig: Config = {
          ...config,
          branchName: currentInput,
        } as Config;
        setStep('complete');
        onConfigComplete(finalConfig);
        break;
    }
  };

  const getPrompt = () => {
    switch (step) {
      case 'url':
        return 'GitLab URL (e.g., https://gitlab.com):';
      case 'token':
        return 'GitLab Personal Access Token:';
      case 'repos':
        return 'Repository paths (comma-separated, e.g., group/repo1, group/repo2):';
      case 'branch':
        return 'Branch name to create:';
      default:
        return '';
    }
  };

  const shouldSkipStep = () => {
    switch (step) {
      case 'url':
        return !!envConfig.GITLAB_URL;
      case 'token':
        return !!envConfig.GITLAB_TOKEN;
      case 'repos':
        return !!envConfig.GITLAB_REPOS;
      default:
        return false;
    }
  };

  if (step === 'complete') {
    return null;
  }

  if (shouldSkipStep()) {
    return null; // This will trigger the useEffect to skip to next step
  }

  return (
    <Box flexDirection="column">
      <Text bold color="cyan">
        🔧 Configuration Setup
      </Text>
      <Text>{''}</Text>

      {missingEnvVars.length > 0 && (
        <>
          <Text color="yellow">
            💡 Missing environment variables: {missingEnvVars.join(', ')}
          </Text>
          <Text color="gray">
            Set these to skip interactive prompts next time.
          </Text>
          <Text>{''}</Text>
        </>
      )}

      <Text>{getPrompt()}</Text>
      {envConfig.GITLAB_URL && step === 'url' && (
        <Text color="green">Using from env: {envConfig.GITLAB_URL}</Text>
      )}
      {envConfig.GITLAB_TOKEN && step === 'token' && (
        <Text color="green">Using token from env</Text>
      )}
      {envConfig.GITLAB_REPOS && step === 'repos' && (
        <Text color="green">Using repos from env: {envConfig.GITLAB_REPOS}</Text>
      )}

      <TextInput
        value={currentInput}
        onChange={setCurrentInput}
        onSubmit={handleSubmit}
        mask={step === 'token'? "*" : ""}
      />
    </Box>
  );
};
