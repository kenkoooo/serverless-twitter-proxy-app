import { Box, Container, makeStyles, Tab, Tabs } from "@material-ui/core";
import React, { useState } from "react";
import { Video } from "../api";

interface Props {
  videos: Video[];
}

const useStyles = makeStyles(() => ({
  videoBox: {
    width: "100%",
  },
}));
export const VideoPreview = (props: Props) => {
  const [tab, setTab] = useState(0);
  const classes = useStyles();

  const videos = props.videos.sort((a, b) => b.bitrate - a.bitrate).slice(0, 3);
  return (
    <Container>
      <Tabs value={tab} onChange={(e, v) => setTab(v)} centered>
        {videos.map((v, i) => (
          <Tab
            key={i}
            label={i === 0 ? "高画質" : i === 1 ? "普通" : "低画質"}
          />
        ))}
      </Tabs>
      <Box>
        <video src={videos[tab].url} controls className={classes.videoBox} />
      </Box>
      <Box display="flex" justifyContent="center">
        <p>画面を長押しで保存！</p>
      </Box>
    </Container>
  );
};
