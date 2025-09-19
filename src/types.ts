export interface Repository {
	id: number;
	name: string;
	web_url: string;
	default_branch: string;
}

export interface Config {
	gitlabUrl: string;
	token: string;
	repositories: string[];
	branchName: string;
}


export interface EnvConfig {
  GITLAB_URL?: string;
  GITLAB_TOKEN?: string;
  GITLAB_REPOS?: string;
}
