import { type SSTConfig } from 'sst'
import { Function, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const api = new Function(stack, 'api', {
		handler: 'src/bin/handlers/api.rs',
		url: { cors: true }
	})
	stack.addOutputs({ url: api.url })
}

export default {
	config(_input) {
		return {
			name: 'parrot-api',
			region: 'us-east-1',
		}
	},
	stacks(app) {
		app.setDefaultFunctionProps({
			runtime: 'rust',
			logRetention: 'one_week',
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