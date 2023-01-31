import { Duration, CfnOutput } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { Function, Runtime, Alias, Version, AssetCode, FunctionUrl, FunctionUrlOptions } from 'aws-cdk-lib/aws-lambda'
import { ILambdaDeploymentConfig, LambdaDeploymentGroup } from 'aws-cdk-lib/aws-codedeploy'
import { LambdaDeploymentConfig } from 'aws-cdk-lib/aws-codedeploy'
import { FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda'

export interface LambdaConstructProps {
  readonly functionName: string
  readonly brazilPackagePath?: string
  readonly deploymentConfig?: ILambdaDeploymentConfig  
  readonly timeout?: Duration
  readonly memorySize?: number
  readonly environment?: { [key: string]: string }
}

export class LambdaConstruct extends Function {
  private readonly id: string;

  constructor(scope: Construct, id: string, props: LambdaConstructProps) {
    super(scope, id, {
      ...props,
      code: new AssetCode(props.brazilPackagePath?? 'target/lambda/release/bootstrap.zip'),
      description: `Generated on: ${new Date().toISOString()}`,
      runtime: Runtime.PROVIDED_AL2,
      handler: 'doesnt.matter'
    })
    this.id = id;

    this.createDeploymentGroup(
      props.functionName, 
      props.deploymentConfig ?? LambdaDeploymentConfig.ALL_AT_ONCE,
      this.currentVersion
    )
  }

  public withFunctionUrl(): LambdaConstruct {
    const functionUrl = this.addFunctionUrl({
      authType: FunctionUrlAuthType.AWS_IAM,
    })

    new CfnOutput(this, `${this.id}-url`, {
      exportName: `${this.id}-url`,
      value: functionUrl.url,
    });

    return this;
  }

  private createDeploymentGroup(functionName: string, config: ILambdaDeploymentConfig, version: Version): LambdaDeploymentGroup {
    const alias = new Alias(this, `${functionName}LambdaAlias`, {
      aliasName: 'live',
      version: version,
    })
    const deploymentGroupName = `${functionName}DeploymentGroup`
    return new LambdaDeploymentGroup(this, deploymentGroupName, {
      alias: alias,
      deploymentGroupName: deploymentGroupName,
      deploymentConfig: config,
      ignorePollAlarmsFailure: true,
      autoRollback: {
        stoppedDeployment: true,
        failedDeployment: true,
      },
    })
  }
}