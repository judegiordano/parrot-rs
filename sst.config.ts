import { type SSTConfig } from 'sst'
import { Function, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const api = new Function(stack, 'api', {
		handler: 'src/bin/handlers/api.rs',
		url: { cors: true },
		logRetention: 'one_week'
	})
	new Function(stack, 'simple-function', {
		handler: 'src/bin/handlers/simple-function.rs',
		logRetention: 'one_week'
	})
	stack.addOutputs({ url: api.url })
}

export default {
	config(_input) {
		return {
			name: 'aws-sst-template',
			region: 'us-east-1',
		}
	},
	stacks(app) {
		app.setDefaultFunctionProps({
			runtime: 'rust',
			architecture: 'arm_64',
			memorySize: '2048 MB',
			timeout: 28,
			environment: {
				STAGE: app.stage,
				REGION: app.region,
				LOG_LEVEL: process.env.LOG_LEVEL,
				MONGO_URI: process.env.MONGO_URI,
			}
		})
		app.stack(ApiStack)
	},
} satisfies SSTConfig
