import React from 'react';
import { Box, Text } from 'ink';

export const EnvInfo: React.FC = () => {
  return (
    <Box flexDirection="column" marginBottom={1}>
      <Text bold color="cyan">Environment Variables:</Text>
      <Text color="gray">GITLAB_URL - Your GitLab instance URL</Text>
      <Text color="gray">GITLAB_TOKEN - Your GitLab personal access token</Text>
      <Text color="gray">GITLAB_REPOS - Comma-separated repository paths</Text>
      <Text>{''}</Text>
      <Text color="yellow">Example:</Text>
      <Text color="gray">export GITLAB_URL="https://gitlab.company.com"</Text>
      <Text color="gray">export GITLAB_TOKEN="glpat-xxxxxxxxxxxx"</Text>
      <Text color="gray">export GITLAB_REPOS="group/repo1,group/repo2"</Text>
    </Box>
  );
};
