import { type SSTConfig } from 'sst'
import { Bucket, Function, Queue, type StackContext } from 'sst/constructs'

function ApiStack({ stack }: StackContext) {
	const createOutputQueue = new Queue(stack, 'create-output-fifo', {
		consumer: 'src/bin/handlers/queues/create-output.rs',
		cdk: { queue: { fifo: true } }
	})
	const trainVoiceQueue = new Queue(stack, 'train-voice-fifo', {
		consumer: 'src/bin/handlers/queues/train-voice.rs',
		cdk: { queue: { fifo: true } }
	})
	const api = new Function(stack, 'api', {
		handler: 'src/bin/handlers/api.rs',
		url: { cors: true }
	})

	const outputBucket = new Bucket(stack, 'outputs', {
		cdk: { bucket: { versioned: true, publicReadAccess: false } }
	})

	const sampleBucket = new Bucket(stack, 'samples', {
		cdk: { bucket: { versioned: true, publicReadAccess: false } }
	})

	sampleBucket.addNotifications(stack, {
		sampleUploaded: {
			function: {
				handler: 'src/bin/handlers/triggers/sample-uploaded.rs',
				timeout: 120,
			},
			events: ['object_created_put'],
			filters: [{ suffix: '.mp3' }],
		}
	})

	const functions = stack.getAllFunctions()
	functions.forEach((fn) => {
		fn.addEnvironment('CREATE_OUTPUT_QUEUE_URL', createOutputQueue.cdk.queue.queueUrl)
		fn.addEnvironment('TRAIN_VOICE_QUEUE_URL', trainVoiceQueue.cdk.queue.queueUrl)
		fn.addEnvironment('SAMPLES_BUCKET_NAME', sampleBucket.bucketName)
		fn.addEnvironment('OUTPUTS_BUCKET_NAME', outputBucket.bucketName)
		fn.attachPermissions(['s3', 'sqs'])
	})
}

export default {
	config(_input) {
		return { name: 'parrot-api', region: 'us-east-1', }
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
				AUTHENTICATION_TOKEN: process.env.AUTHENTICATION_TOKEN,
				ELEVEN_LABS_API_KEY: process.env.ELEVEN_LABS_API_KEY,
			}
		})
		app.stack(ApiStack)
	}
} satisfies SSTConfig
