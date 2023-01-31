import { App } from 'aws-cdk-lib';
import { PlaygroundApiStack } from './lib/stacks/playground_api_stack';
import { ENVIRONMENTS } from './lib/config/environments';

const app = new App();

ENVIRONMENTS.forEach(environment => {
    new PlaygroundApiStack(app, `${environment.name}-playground-api-stack`, {
        env: {
            region: environment.region,
            account: environment.account
        },
        prefix: environment.name,
        isDev: environment.isDev
    });
})

app.synth();