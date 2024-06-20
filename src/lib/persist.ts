import { SYNC_EVENT_NAME } from "./core";

const PREFIX = "hoyo-game-starter";

export function getStorage(key: keyof typeof GameKey): HoyoInterface {
  const item = window.localStorage.getItem(`${PREFIX}-${key}`);
  let hoyo: HoyoInterface = {
    launcherPath: "",
    launcherFile: "",
    gamePath: "",
    gameFile: "",
    scriptPath: "",
    scriptFile: "",
  };
  if (item != null) {
    hoyo = JSON.parse(item);
  }
  return hoyo;
}

export function setStorage(
  key: keyof typeof GameKey,
  hoyo: HoyoInterface
): void {
  window.localStorage.setItem(`${PREFIX}-${key}`, JSON.stringify(hoyo));
}

export function getSyncStatus() {
  const status = window.localStorage.getItem(SYNC_EVENT_NAME);
  return status === "true";
}

export function setSyncStatus(status: boolean) {
  window.localStorage.setItem(SYNC_EVENT_NAME, String(status));
}
