import { App, Stack, StackProps } from 'aws-cdk-lib';
import { LambdaConstruct } from '../constructs/lambda_construct';
import { FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda'

export interface PlaygroundApiStackProps extends StackProps {
    readonly prefix: string
}

export class PlaygroundApiStack extends Stack {    
    constructor(parent: App, name: string, props: PlaygroundApiStackProps) {
        super(parent, name, props);
        
        const lambda_name = `${props.prefix}-playground-lambda-api`
        new LambdaConstruct(this, lambda_name, {
            functionName: lambda_name,
            brazilPackagePath: 'target/lambda/playground-api/bootstrap.zip',
        }).withFunctionUrl()
    }
}