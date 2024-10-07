import * as path from 'path';
import * as cdk from 'aws-cdk-lib';
import {Construct} from 'constructs';
import {RustFunction} from 'cargo-lambda-cdk';

export class MomentoRustLambdaPerfStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const momentoApiKeyParam = new cdk.CfnParameter(this, 'MomentoApiKey', {
      type: 'String',
      description: 'The Momento API key that will be used to read from the cache.',
      noEcho: true,
    });

    new RustFunction(this, 'PerfLambdaRust', {
      functionName: 'PerfLambdaRust',
      runtime: 'provided.al2023',
      manifestPath: path.join(__dirname, '../../lambda/rust-perf-lambda/Cargo.toml'),
      timeout: cdk.Duration.seconds(300),
      memorySize: 128,
      environment: {
        MOMENTO_API_KEY: momentoApiKeyParam.valueAsString,
      },
    });

    new cdk.aws_lambda_nodejs.NodejsFunction(this, 'PerfLambdaNodejs', {
      functionName: 'PerfLambdaNodejs',
      runtime: cdk.aws_lambda.Runtime.NODEJS_20_X,
      entry: path.join(__dirname, '../../lambda/nodejs-perf-lambda/handler.ts'),
      projectRoot: path.join(__dirname, '../../lambda/nodejs-perf-lambda'),
      depsLockFilePath: path.join(__dirname, '../../lambda/nodejs-perf-lambda/package-lock.json'),
      handler: 'handler',
      timeout: cdk.Duration.seconds(300),
      memorySize: 128,
      environment: {
        MOMENTO_API_KEY: momentoApiKeyParam.valueAsString,
      },
    });

    new cdk.aws_lambda_nodejs.NodejsFunction(this, 'PerfLambdaNodejsNapi', {
      functionName: 'PerfLambdaNodejsNapi',
      runtime: cdk.aws_lambda.Runtime.NODEJS_20_X,
      entry: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda/handler.ts'),
      projectRoot: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda'),
      depsLockFilePath: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda/package-lock.json'),
      handler: 'handler',
      timeout: cdk.Duration.seconds(300),
      memorySize: 128,
      environment: {
        MOMENTO_API_KEY: momentoApiKeyParam.valueAsString,
      },
    });
  }
}
