import React, { useState, useEffect } from 'react';
import { ConfigInput } from './components/ConfigInput.js';
import { BranchCreator } from './components/BranchCreator.js';
import { Config } from './types.js';
import { loadEnvConfig, validateEnvConfig } from './env.js';

export const App: React.FC = () => {
  const [config, setConfig] = useState<Config | null>(null);
  const [envConfig] = useState(loadEnvConfig());
  const [envValidation] = useState(validateEnvConfig(envConfig));

  // Auto-configure if all env vars are present
  useEffect(() => {
    if (envValidation.isValid && envConfig.GITLAB_REPOS) {
      // Still need to ask for branch name
      if (!config) {
        return;
      }
    }
  }, [envValidation, envConfig, config]);


  if (!config) {
    return (
      <ConfigInput
        onConfigComplete={setConfig}
        envConfig={envConfig}
        missingEnvVars={envValidation.missing}
      />
    );
  }

  return <BranchCreator config={config} />;
};
