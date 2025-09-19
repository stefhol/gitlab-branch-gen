import {useState, useEffect} from 'react';
import {Box, Text} from 'ink';
import {GitLabAPI} from '../gitlab-api.js';
import {Config} from '../types.js';

interface Props {
	config: Config;
}

interface Result {
	repository: string;
	status: 'success' | 'exists' | 'error';
	url?: string;
	error?: string;
}

export function BranchCreator({config}: Props) {
	const [results, setResults] = useState<Result[]>([]);
	const [processing, setProcessing] = useState(true);
	const [_currentRepo, setCurrentRepo] = useState<string>('');

	useEffect(() => {
		const createBranches = async () => {
			const api = new GitLabAPI(config.gitlabUrl, config.token);
			const newResults: Result[] = [];

			for (const repoPath of config.repositories) {
				setCurrentRepo(repoPath);

				try {
					const repo = await api.getRepository(repoPath);
					const branchExists = await api.branchExists(
						repo.id,
						config.branchName,
					);

					if (branchExists) {
						newResults.push({
							repository: repoPath,
							status: 'exists',
							url: `${repo.web_url}/-/tree/${config.branchName}`,
						});
					} else {
						await api.createBranch(
							repo.id,
							config.branchName,
							repo.default_branch,
						);
						newResults.push({
							repository: repoPath,
							status: 'success',
							url: `${repo.web_url}/-/tree/${config.branchName}`,
						});
					}
				} catch (error: any) {
					newResults.push({
						repository: repoPath,
						status: 'error',
						error: error.message,
					});
				}

				setResults([...newResults]);
			}

			setProcessing(false);
		};

		createBranches();
	}, [config]);

	return (
		<Box flexDirection="column">
			<Text bold color="cyan">
				🚀 GitLab Branch Creator
			</Text>
			<Text>{''}</Text>

			{processing && (
				<Box>
					<Text color="yellow">Processing</Text>
				</Box>
			)}

			{results.map((result, index) => (
				<Box key={index} flexDirection="column" marginBottom={1}>
					<Box>
						<Text bold>{result.repository}</Text>
						{result.status === 'success' && (
							<Text color="green"> ✓ Branch created</Text>
						)}
						{result.status === 'exists' && (
							<Text color="yellow"> ⚠ Branch already exists</Text>
						)}
						{result.status === 'error' && (
							<Text color="red"> ✗ Error: {result.error}</Text>
						)}
					</Box>
					{result.url && (
						<Text color="blue" dimColor>
							└─ {result.url}
						</Text>
					)}
				</Box>
			))}

			{!processing && (
				<>
					<Text>{''}</Text>
					<Text color="green">
						✨ Done! Created branch "{config.branchName}" in{' '}
						{results.filter(r => r.status === 'success').length} repositories
					</Text>
				</>
			)}
		</Box>
	);
}
