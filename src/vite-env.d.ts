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

interface AppointFile {
  game: string;
  exe: string;
}

interface Hoyo extends AppointFile {
  root: string;
  launcher: string;
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

interface PickFolder extends InvokeParam {
  key: keyof typeof GameKey;
  title: string;
}

interface CheckPath extends InvokeParam {
  dir: string;
  file: string;
}

interface GameProcess extends InvokeParam {
  process: string;
}

interface WindowVisible extends InvokeParam {
  status: boolean;
}

interface LocalVersion extends InvokeParam {
  key: keyof typeof GameKey;
  installPath: string;
}
