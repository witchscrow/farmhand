import { env } from '$env/dynamic/private';

type Video = {
	id: string;
	title: string;
	status: string;
	playlist: string;
	created_at: string;
	updated_at: string;
};

type RequestedVideo = {
	id: string;
	title: string;
	processing_status: string;
	video_path: string;
	created_at: string;
	updated_at: string;
};

enum VideoError {
	FETCH_ERROR = 'FETCH_ERROR',
	NOT_FOUND = 'NOT_FOUND',
	UNKNOWN = 'UNKNOWN'
}

export const fetchVideo = async (videoID: string): Promise<Video | null> => {
	try {
		const response = await fetch(`${env.API_URL}/video?id=${videoID}`);

		if (response.ok) {
			const videoData: { videos: RequestedVideo[] } = await response.json();
			if (videoData.videos) {
				const video = videoData.videos[0];
				return {
					id: video.id,
					status: video.processing_status,
					title: video.title,
					playlist: `${env.API_URL}/${video.video_path}`,
					created_at: video.created_at,
					updated_at: video.updated_at
				};
			} else {
				throw VideoError.NOT_FOUND;
			}
		} else {
			throw VideoError.FETCH_ERROR;
		}
	} catch (e) {
		if (e === VideoError.FETCH_ERROR) {
			throw e;
		}
		throw VideoError.UNKNOWN;
	}
};

export const fetchVideos = async (): Promise<Video[]> => {
	try {
		const response = await fetch(`${env.API_URL}/video`);

		if (response.ok) {
			const videoData: { videos: RequestedVideo[] } = await response.json();
			if (videoData.videos) {
				return videoData.videos.map((video) => ({
					id: video.id,
					status: video.processing_status,
					title: video.title,
					playlist: `${env.API_URL}/${video.video_path}`,
					created_at: video.created_at,
					updated_at: video.updated_at
				}));
			} else {
				throw VideoError.NOT_FOUND;
			}
		} else {
			throw VideoError.FETCH_ERROR;
		}
	} catch (e) {
		if (e === VideoError.FETCH_ERROR) {
			throw e;
		}
		throw VideoError.UNKNOWN;
	}
};
