/// <reference types="svelte" />
/// <reference types="vite/client" />

declare enum GameKey {
  bh3 = "bh3",
  ys = "ys",
  star = "star",
  zzz = "zzz",
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
    game_packages: [
      {
        game: {
          id: string;
          biz: string;
        };
        main: {
          major: {
            version: string;
          };
          patches: unknown[];
        };
        pre_download?: {
          major?: object;
          patches: unknown[];
        };
      }
    ];
  };
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
