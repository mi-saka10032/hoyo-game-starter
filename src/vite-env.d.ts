/// <reference types="svelte" />
/// <reference types="vite/client" />

declare enum GameKey {
  bh3 = "bh3",
  ys = "ys",
  star = "star",
}

interface GameItem {
  key: keyof typeof GameKey;
  bg: string;
  gameEnName: string;
  gameCnName: string;
  processName: string;
}

interface HoyoInterface {
  launcherPath: string;
  launcherFile: string;
  gamePath: string;
  gameFile: string;
  scriptPath: string;
  scriptFile: string;
}

interface StaticResource {
  data: {
    game: {
      latest: {
        version: string
      }
    }
    pre_download_game?: object
  }
}

interface InvokeParam {
  [key: string]: any;
}

interface FileProp extends InvokeParam {
  path: string;
  file: string;
}

interface TauriWindowEvent<T> {
  event: string;
  id: number;
  payload: T;
  windowLabel: string;
}