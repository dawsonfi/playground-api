export interface Environment {
    readonly name: string
    readonly account: string
    readonly region: string    
}

const DEVO_ENVIRONMENT: Environment = {
    name: process.env.USER || 'undefined',
    account: process.env.PLAYGROUND_AWS_ACCOUNT_ID || 'undefined',
    region: 'us-west-2'
}

const BETA_ENVIRONMENT: Environment = {
    name: 'beta',
    account: process.env.PLAYGROUND_AWS_ACCOUNT_ID || 'undefined',
    region: 'us-west-2'
}

const PROD_ENVIRONMENT: Environment = {
    name: 'prod',
    account: process.env.PLAYGROUND_AWS_ACCOUNT_ID || 'undefined',
    region: 'us-east-1'
}

export const ENVIRONMENTS = [
    DEVO_ENVIRONMENT,
    BETA_ENVIRONMENT,
    PROD_ENVIRONMENT
]