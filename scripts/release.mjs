import { execSync } from 'child_process';

execSync('npx changeset tag', { stdio: 'inherit' });
