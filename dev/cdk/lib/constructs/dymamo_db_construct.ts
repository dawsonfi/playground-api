import { Construct } from 'constructs'
import { ITable, Table, Attribute, BillingMode } from 'aws-cdk-lib/aws-dynamodb'

export interface DynamoDBConstructProps {
  readonly tableName: string
  readonly partitionKey: Attribute
  readonly billingMode?: BillingMode
  readonly shouldReuse?: boolean
}

export class DynamoDBConstruct extends Construct {
  public readonly table: ITable;

  constructor(scope: Construct, id: string, props: DynamoDBConstructProps) {
    super(scope, id)

    if (props.shouldReuse) {
      this.table = Table.fromTableName(this, id, props.tableName);
    } else {
      this.table = new Table(this, id, props)
    }
  }
}