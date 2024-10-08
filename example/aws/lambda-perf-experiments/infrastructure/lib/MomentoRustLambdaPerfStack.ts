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
      architecture: cdk.aws_lambda.Architecture.ARM_64,
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
      architecture: cdk.aws_lambda.Architecture.ARM_64,
      entry: path.join(__dirname, '../../lambda/nodejs-perf-lambda/handler.ts'),
      projectRoot: path.join(__dirname, '../../lambda/nodejs-perf-lambda'),
      depsLockFilePath: path.join(__dirname, '../../lambda/nodejs-perf-lambda/package-lock.json'),
      handler: 'handler',
      timeout: cdk.Duration.seconds(300),
      memorySize: 8192,
      environment: {
        MOMENTO_API_KEY: momentoApiKeyParam.valueAsString,
      },
    });

    new cdk.aws_lambda_nodejs.NodejsFunction(this, 'PerfLambdaNodejsNapi', {
      functionName: 'PerfLambdaNodejsNapi',
      runtime: cdk.aws_lambda.Runtime.NODEJS_20_X,
      architecture: cdk.aws_lambda.Architecture.ARM_64,
      bundling: {
        volumes: [
          {
            hostPath: path.join(__dirname, '../../lambda/napi-rs-workspace'),
            containerPath: '/napi-rs-workspace',
          },
        ],
        forceDockerBundling: true,
        // esbuildArgs: {'--packages': 'bundle'},
        // esbuildVersion: '0.24.0',
        // nodeModules: ['weather_cacher@0.0.0'],
        externalModules: ['weather_cacher'],
        commandHooks: {
          beforeInstall(): string[] {
            return [];
          },
          beforeBundling(inputDir: string, outputDir: string): string[] {
            return [
              // `cp -r ${path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda/node_modules/weather_cacher')} ${path.join(outputDir, 'node_modules')}`,
              // `ls -l ${inputDir}/node_modules`,
              // 'ls -l ../..',
              // 'ls -l /napi-rs-workspace',
              // 'rustup --help',
              // 'cd /napi-rs-workspace/weather_cacher && yarn build',
              // `cp -r /napi-rs-workspace/weather_cacher ${path.join(outputDir, 'node_modules')}`,
            ];
          },
          afterBundling(): string[] {
            return [];
          },
        },
      },
      entry: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda/handler.ts'),
      projectRoot: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda'),
      depsLockFilePath: path.join(__dirname, '../../lambda/nodejs-napi-perf-lambda/package-lock.json'),
      handler: 'handler',
      timeout: cdk.Duration.seconds(300),
      memorySize: 8192,
      environment: {
        MOMENTO_API_KEY: momentoApiKeyParam.valueAsString,
      },
    });
  }
}
