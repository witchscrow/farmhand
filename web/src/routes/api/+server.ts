// import type { RequestHandler } from '../(guest)/health/$types';

// interface UploadRequest {
// 	parts: number;
// 	fileKey: string;
// 	command: string;
// }

// export const POST: RequestHandler = async ({ locals, request }) => {
// 	try {
// 		const user = locals.user;
// 		if (!user) return new Response('Unauthorized', { status: 401 });

// 		const data = (await request.json()) as UploadRequest;
// 		const { fileKey, parts, command } = data;
// 		if (!command) {
// 			return new Response('Bad request, command required', { status: 400 });
// 		}

// 		// If the command is not start or finish, return a 501
// 		if (command !== 'start' && command !== 'finish') {
// 			return new Response('Not implemented', { status: 501 });
// 		}

// 		if (command === 'start') {
// 			// The start command requires a fileKey and parts
// 			if (!fileKey || !parts) {
// 				return new Response('Bad request, fileKey and parts required', { status: 400 });
// 			}
// 			// Start the upload and return the uploadId and pre-signed URLs
// 			const { uploadId, urls } = await startUpload(fileKey, parts);
// 			return new Response(JSON.stringify({ uploadId, urls }));
// 		}

// 		if (command === 'finish') {
// 			// The finish command requires an uploadId, fileKey, and etags
// 			if (!uploadId || !fileKey) {
// 				return new Response('Bad request, uploadId, fileKey, and etags required', { status: 400 });
// 			}
// 			// Finish the upload and return the version id
// 			await finishUpload(uploadId, fileKey, etags);
// 			return new Response('OK');
// 		}

// 		throw new Error('Made it to the end of the upload handler without returning a response');
// 	} catch (error) {
// 		return new Response(`Error uploading file ${error}`, { status: 500 });
// 	}
// };
