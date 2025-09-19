#!/usr/bin/env node
import 'dotenv/config';
import {render} from 'ink';
import meow from 'meow';
import {App} from './App.js';

const cli = meow(
    `
    Usage
      $ branch-gen

	Environment Variables
	GITLAB_URL="https://gitlab.com"
	GITLAB_TOKEN="glpat...."
	GITLAB_REPOS="group/repo1, group/repo2
    `,
{
        importMeta: import.meta,
        flags:  {}
    },
);
if (process.argv.length <= 2) {
	cli.showHelp();
} else {
	render(<App   />);
}
