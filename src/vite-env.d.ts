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

interface GamePackage {
  url: string;
  md5: string;
  size: string; // -> number
  decompressed_size: string; // -> number
}

interface AudioPackage extends GamePackage {
  language: string;
}

interface GamePatch {
  version: string;
  game_pkgs: GamePackage[];
  audio_pkgs: AudioPackage[];
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
          patches: GamePatch[];
        };
        pre_download?: {
          major: object | null;
          patches: GamePatch[];
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
