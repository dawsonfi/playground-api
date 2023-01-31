import { App, Stack, StackProps } from 'aws-cdk-lib';
import { LambdaConstruct } from '../constructs/lambda_construct';
import { LambdaDeploymentConfig } from 'aws-cdk-lib/aws-codedeploy'

export interface PlaygroundApiStackProps extends StackProps {
    readonly prefix: string
    readonly isDev: boolean
}

export class PlaygroundApiStack extends Stack {
    constructor(parent: App, name: string, props: PlaygroundApiStackProps) {
        super(parent, name, props);

        const lambda_name = `${props.prefix}-playground-lambda-api`
        new LambdaConstruct(this, lambda_name, {
            functionName: lambda_name,
            brazilPackagePath: 'target/lambda/playground-api',
            deploymentConfig: props.isDev? LambdaDeploymentConfig.ALL_AT_ONCE : LambdaDeploymentConfig.CANARY_10PERCENT_10MINUTES
        }).withFunctionUrl()
    }
}