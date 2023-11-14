import { type SSTConfig } from 'sst'
import { Function, Queue, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const cloneVoiceQueue = new Queue(stack, 'clone-voice-fifo', {
		consumer: 'src/bin/handlers/queues/clone-voice.rs',
		cdk: { queue: { fifo: true } }
	})
	const api = new Function(stack, 'api', {
		handler: 'src/bin/handlers/api.rs',
		url: { cors: true }
	})
	api.attachPermissions(['sqs'])

	const functions = stack.getAllFunctions()
	functions.forEach((fn) => {
		fn.addEnvironment('CLONE_VOICE_QUEUE_URL', cloneVoiceQueue.cdk.queue.queueUrl)
	})
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
				ELEVEN_LABS_API_KEY: process.env.ELEVEN_LABS_API_KEY,
			}
		})
		app.stack(ApiStack)
	},
} satisfies SSTConfig
