const LOOKUP = "/api/lookup";

interface VideoInfo {
  variants: {
    bitrate?: number;
    url: string;
  }[];
}
export interface Video {
  bitrate: number;
  url: string;
}

export const fetchVideoInfo = async (statusId: BigInt) => {
  const response = await fetch(LOOKUP, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: `{"status_id":${statusId}}`,
  });

  if (response.status === 404) {
    return undefined;
  }

  const body = await response.json();
  if (response.status !== 200) {
    throw body;
  }

  const videos = [] as Video[];
  (body as VideoInfo).variants.forEach(({ bitrate, url }) => {
    if (bitrate) {
      videos.push({ bitrate: bitrate, url: url });
    }
  });

  if (videos.length === 0) {
    return undefined;
  } else {
    return videos;
  }
};
