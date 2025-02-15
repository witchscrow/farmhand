export function GET() {
	return new Response('OK', {
		status: 200,
		headers: {
			'Content-Type': 'text/plain'
		}
	});
}
