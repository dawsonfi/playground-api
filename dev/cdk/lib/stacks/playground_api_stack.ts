import { App, Stack, StackProps } from 'aws-cdk-lib';
import { LambdaConstruct } from '../constructs/lambda_construct';
import { DynamoDBConstruct } from '../constructs/dymamo_db_construct';
import { AttributeType } from 'aws-cdk-lib/aws-dynamodb';
import { LambdaDeploymentConfig } from 'aws-cdk-lib/aws-codedeploy'

export interface PlaygroundApiStackProps extends StackProps {
    readonly prefix: string
    readonly isDev: boolean
}

export class PlaygroundApiStack extends Stack {
    constructor(parent: App, name: string, props: PlaygroundApiStackProps) {
        super(parent, name, props);

        const lambda_name = `${props.prefix}-playground-lambda-api`
        const playground_api_lambda = new LambdaConstruct(this, lambda_name, {
            functionName: lambda_name,
            brazilPackagePath: 'target/lambda/playground-api',
            isDev: props.isDev
        }).withFunctionUrl()

        const tables = [
            {
                tableName: 'Account',
                partitionKey: {
                    name: 'id',
                    type: AttributeType.STRING
                }
            }
        ];

        tables.forEach(table => {
            const created_table = new DynamoDBConstruct(this, `${table.tableName}Table`, {
                tableName: table.tableName,
                partitionKey: table.partitionKey,
                shouldReuse: props.isDev
            });

            created_table.table.grantReadWriteData(playground_api_lambda);
        });
    }
}