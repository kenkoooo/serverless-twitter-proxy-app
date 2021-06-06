import {
  Button,
  CircularProgress,
  Container,
  makeStyles,
  TextField,
  Typography,
} from "@material-ui/core";
import Alert from "@material-ui/lab/Alert";
import React, { useState } from "react";
import { fetchVideoInfo, Video } from "./api";
import { VideoPreview } from "./component/VideoPreview";
import { parseUrl } from "./parser";

const useStyles = makeStyles((theme) => ({
  form: {
    marginTop: theme.spacing(8),
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
  text: {
    margin: theme.spacing(1),
  },
  preview: {
    margin: theme.spacing(1),
  },
}));

type VideoFetchState = "NotFound" | "Error" | "Pending" | null | Video[];

const PreviewContainer = (props: { fetchedState: VideoFetchState }) => {
  if (!props.fetchedState) {
    return null;
  }
  if (props.fetchedState === "Pending") {
    return <CircularProgress />;
  }
  if (props.fetchedState === "NotFound") {
    return <Alert severity="warning">動画が見つかりませんでした</Alert>;
  }
  if (props.fetchedState === "Error") {
    return <Alert severity="error">エラー</Alert>;
  }
  return <VideoPreview videos={props.fetchedState} />;
};

export const App = () => {
  const classes = useStyles();
  const [inputText, setInputText] = useState("");
  const [changed, setChanged] = useState(false);
  const parsedStatusId = parseUrl(inputText);

  const [fetchedVideoInfo, setFetchedVideoInfo] =
    useState<VideoFetchState>(null);

  const fetchVideo = () => {
    if (!parsedStatusId) {
      return;
    }
    setChanged(false);
    setFetchedVideoInfo("Pending");
    fetchVideoInfo(parsedStatusId)
      .then((response) => {
        if (response) {
          setFetchedVideoInfo(response);
        } else {
          setFetchedVideoInfo("NotFound");
        }
      })
      .catch((e) => {
        console.error(e);
        setFetchedVideoInfo("Error");
      });
  };

  return (
    <Container component="main" maxWidth="xs">
      <div className={classes.form}>
        <Typography component="h1" variant="h5">
          ツイッター動画保存
        </Typography>
        <TextField
          className={classes.text}
          variant="outlined"
          fullWidth
          placeholder="ツイートのURLをペースト！"
          value={inputText}
          onChange={(e) => {
            setInputText(e.target.value);
            setChanged(true);
          }}
          onKeyPress={(e) => {
            if (e.key === "Enter") {
              fetchVideo();
            }
          }}
        />
        <div className={classes.preview}>
          <PreviewContainer fetchedState={fetchedVideoInfo} />
        </div>
        <Button
          color="primary"
          variant="contained"
          disabled={
            !parsedStatusId || fetchedVideoInfo === "Pending" || !changed
          }
          onClick={fetchVideo}
        >
          ダウンロード！
        </Button>
      </div>
    </Container>
  );
};
