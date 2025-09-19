import axios, {AxiosInstance} from 'axios';
import {Repository} from './types.js';

export class GitLabAPI {
	private client: AxiosInstance;

	constructor(baseURL: string, token: string) {
		this.client = axios.create({
			baseURL: `${baseURL}/api/v4`,
			headers: {
				'PRIVATE-TOKEN': token,
			},
		});
	}

	async getRepository(projectPath: string): Promise<Repository> {
		const encodedPath = encodeURIComponent(projectPath);
		const response = await this.client.get(`/projects/${encodedPath}`);
		return response.data;
	}

	async createBranch(
		projectId: number,
		branchName: string,
		ref: string,
	): Promise<void> {
		await this.client.post(`/projects/${projectId}/repository/branches`, {
			branch: branchName,
			ref,
		});
	}

	async branchExists(projectId: number, branchName: string): Promise<boolean> {
		try {
			await this.client.get(
				`/projects/${projectId}/repository/branches/${branchName}`,
			);
			return true;
		} catch (error: any) {
			if (error.response?.status === 404) {
				return false;
			}
			throw error;
		}
	}
}
