import { Duration, CfnOutput } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import {Function, Runtime, Alias, Version, AssetCode, FunctionUrl, FunctionUrlOptions, Architecture} from 'aws-cdk-lib/aws-lambda'
import { ILambdaDeploymentConfig, LambdaDeploymentGroup } from 'aws-cdk-lib/aws-codedeploy'
import { LambdaDeploymentConfig } from 'aws-cdk-lib/aws-codedeploy'
import { FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda'

export interface LambdaConstructProps {
  readonly functionName: string
  readonly brazilPackagePath: string
  readonly timeout?: Duration
  readonly memorySize?: number
  readonly environment?: { [key: string]: string }
  readonly isDev?: boolean
}

export class LambdaConstruct extends Function {
  private readonly id: string;
  private readonly isDev: boolean;

  constructor(scope: Construct, id: string, props: LambdaConstructProps) {
    super(scope, id, {
      ...props,
      code: new AssetCode(props.brazilPackagePath),
      description: `Generated on: ${new Date().toISOString()}`,
      runtime: Runtime.PROVIDED_AL2,
      architecture: Architecture.ARM_64,
      handler: 'doesnt.matter'
    })
    this.id = id;
    this.isDev = props.isDev ?? false;

    this.createDeploymentGroup(
      props.functionName,
      this.isDev? LambdaDeploymentConfig.ALL_AT_ONCE : LambdaDeploymentConfig.CANARY_10PERCENT_10MINUTES,
      this.currentVersion
    )
  }

  public withFunctionUrl(): LambdaConstruct {
    const functionUrl = this.addFunctionUrl({
      authType: this.isDev? FunctionUrlAuthType.NONE : FunctionUrlAuthType.AWS_IAM,
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